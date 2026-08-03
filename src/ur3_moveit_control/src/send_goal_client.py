#!/usr/bin/env python3

import csv
import math
import random
from pathlib import Path

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient

from builtin_interfaces.msg import Duration
from control_msgs.action import FollowJointTrajectory
from sensor_msgs.msg import JointState
from trajectory_msgs.msg import JointTrajectoryPoint
from ur3_moveit_control.action import UR3Control


# Fixed bottle pose in base_link. The Gazebo launch file spawns pick_box at
# this same position, so the grasp test is repeatable from one run to the next.
FIXED_BOTTLE_X = 0.62
FIXED_BOTTLE_Y = 0.0
FIXED_BOTTLE_Z = 0.05

# Side-grasp tuning for the 0.10 m diameter, 0.26 m tall bottle.
FIXED_APPROACH_CLEARANCE = 0.12
FIXED_TCP_GRASP_OFFSET = 0.06
FIXED_GRASP_DEPTH_OFFSET = 0.015
FIXED_LIFT_OFFSET = 0.15
FIXED_OPEN_POSITION = 0.06
# This asks both fingers to close slightly through the nominal bottle diameter
# so Gazebo establishes contact instead of leaving a visible gap.
FIXED_CLOSE_POSITION = 0.042
FIXED_MAX_EFFORT = 100.0
FIXED_TRANSPORT_Y_MIN = 0.06
FIXED_TRANSPORT_Y_MAX = 0.10
FIXED_TRANSPORT_X_MAX = 0.02
SUCCESSFUL_WAYPOINTS_FILE = Path('successful_grasp_waypoints.csv')

ARM_JOINT_NAMES = [
    'shoulder_pan_joint',
    'shoulder_lift_joint',
    'elbow_joint',
    'wrist_1_joint',
    'wrist_2_joint',
    'wrist_3_joint',
]


def radial_side_grasp_geometry(
        object_x,
        object_y,
        approach_clearance,
        grasp_depth_offset):
    """Calculate a horizontal approach directed radially toward the bottle."""

    radius = math.hypot(object_x, object_y)
    if radius < 1e-6:
        raise ValueError('Object cannot be placed at the base_link origin')

    direction_x = object_x / radius
    direction_y = object_y / radius
    grasp_x = object_x - grasp_depth_offset * direction_x
    grasp_y = object_y - grasp_depth_offset * direction_y
    approach_x = grasp_x - approach_clearance * direction_x
    approach_y = grasp_y - approach_clearance * direction_y

    # q=(0.5, 0.5, 0.5, 0.5) points TCP +Z along base_link +X. Apply a
    # base-link Z yaw so the gripper points radially toward the object.
    yaw = math.atan2(object_y, object_x)
    cosine = math.cos(0.5 * yaw)
    sine = math.sin(0.5 * yaw)
    quaternion = (
        0.5 * (cosine - sine),
        0.5 * (cosine + sine),
        0.5 * (cosine + sine),
        0.5 * (cosine - sine),
    )

    return {
        'approach_x': approach_x,
        'approach_y': approach_y,
        'advance_x': approach_clearance * direction_x,
        'advance_y': approach_clearance * direction_y,
        'grasp_x': grasp_x,
        'grasp_y': grasp_y,
        'quaternion': quaternion,
        'yaw': yaw,
    }


class UR3ActionClient(Node):

    GRIPPER_MIN_POSITION = 0.0
    # Keep a small margin from the simulated prismatic joint's hard stop.
    GRIPPER_MAX_POSITION = 0.06

    def __init__(self):
        super().__init__('ur3_action_client')

        # Action client for controlling the UR3 arm
        self._arm_action_client = ActionClient(
            self,
            UR3Control,
            'ur3_control'
        )

        # Action client for controlling the gripper
        self._gripper_action_client = ActionClient(
            self,
            FollowJointTrajectory,
            '/gripper_controller/follow_joint_trajectory'
        )
        self._pending_lift_offset = None
        self._gripper_completion_timer = None
        self._pick_sequence = None
        self._latest_gripper_effort = None
        self._peak_gripper_effort = 0.0
        self._latest_joint_positions = {}
        self._joint_state_subscription = self.create_subscription(
            JointState,
            '/joint_states',
            self.joint_state_callback,
            10
        )

    def joint_state_callback(self, msg):
        """Track measured gripper effort reported by ros2_control."""

        self._latest_joint_positions = dict(zip(msg.name, msg.position))

        try:
            joint_index = msg.name.index('gripper_joint')
        except ValueError:
            return

        if joint_index >= len(msg.effort):
            return

        measured_effort = float(msg.effort[joint_index])
        self._latest_gripper_effort = measured_effort
        self._peak_gripper_effort = max(
            self._peak_gripper_effort,
            abs(measured_effort)
        )

    def capture_pregrasp_joints(self):
        """Keep the current six arm joints in memory until the trial passes."""

        missing = [
            name for name in ARM_JOINT_NAMES
            if name not in self._latest_joint_positions
        ]
        if missing:
            self.get_logger().warning(
                'Cannot capture pre-grasp waypoint; missing joint states: '
                + ', '.join(missing)
            )
            return

        joints = [
            self._latest_joint_positions[name]
            for name in ARM_JOINT_NAMES
        ]
        self._pick_sequence['captured_pregrasp_joints'] = joints
        self.get_logger().info(
            'Captured pre-grasp joints in memory: '
            + '[' + ', '.join(f'{value:.9f}' for value in joints) + ']'
        )

    def save_successful_waypoint(self):
        """Append the captured pre-grasp waypoint after full trial success."""

        joints = self._pick_sequence.get('captured_pregrasp_joints')
        if joints is None:
            self.get_logger().warning(
                'Trial succeeded, but no pre-grasp joint snapshot was captured'
            )
            return

        qx, qy, qz, qw = self._pick_sequence['grasp_quaternion']
        row = {
            'ros_time_ns': self.get_clock().now().nanoseconds,
            'object_x': self._pick_sequence['object_x'],
            'object_y': self._pick_sequence['object_y'],
            'object_z': self._pick_sequence['object_z'],
            'pregrasp_x': self._pick_sequence['approach_x'],
            'pregrasp_y': self._pick_sequence['approach_y'],
            'pregrasp_z': self._pick_sequence['grasp_z'],
            'qx': qx,
            'qy': qy,
            'qz': qz,
            'qw': qw,
            **dict(zip(ARM_JOINT_NAMES, joints)),
            'transport_dx': self._pick_sequence['transport_dx'],
            'transport_dy': self._pick_sequence['transport_dy'],
        }

        path = SUCCESSFUL_WAYPOINTS_FILE.expanduser().resolve()
        write_header = not path.exists() or path.stat().st_size == 0
        with path.open('a', newline='', encoding='utf-8') as csv_file:
            writer = csv.DictWriter(csv_file, fieldnames=row.keys())
            if write_header:
                writer.writeheader()
            writer.writerow(row)

        self.get_logger().info(
            f'Saved successful pre-grasp waypoint to {path}'
        )

    # =========================================================
    # ARM COMMANDS
    # =========================================================

    def send_pose_goal(self, x, y, z, qx, qy, qz, qw):
        """Send a Cartesian pose goal to the UR3 arm."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.MOVE_POSE

        goal_msg.pose_goal.header.frame_id = 'base_link'
        goal_msg.pose_goal.header.stamp = (
            self.get_clock().now().to_msg()
        )

        goal_msg.pose_goal.pose.position.x = float(x)
        goal_msg.pose_goal.pose.position.y = float(y)
        goal_msg.pose_goal.pose.position.z = float(z)

        goal_msg.pose_goal.pose.orientation.x = float(qx)
        goal_msg.pose_goal.pose.orientation.y = float(qy)
        goal_msg.pose_goal.pose.orientation.z = float(qz)
        goal_msg.pose_goal.pose.orientation.w = float(qw)

        if not self._arm_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The UR3 action server is not available'
            )
            return

        self.get_logger().info(
            f'Sending pose goal: '
            f'x={x:.3f}, y={y:.3f}, z={z:.3f}, '
            f'quaternion=({qx:.3f}, {qy:.3f}, '
            f'{qz:.3f}, {qw:.3f})'
        )

        future = self._arm_action_client.send_goal_async(
            goal_msg
        )

        future.add_done_callback(
            self.arm_goal_response_callback
        )

    def send_home_goal(self):
        """Send the robot to its predefined home configuration."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.MOVE_HOME

        if not self._arm_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The UR3 action server is not available'
            )
            return

        self.get_logger().info('Sending home goal...')

        future = self._arm_action_client.send_goal_async(
            goal_msg
        )

        future.add_done_callback(
            self.arm_goal_response_callback
        )

    def send_joint_goal(self, j1, j2, j3, j4, j5, j6):
        """Send a six-joint position goal to the UR3 arm."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.MOVE_JOINT

        goal_msg.joint_goal.position = [
            float(j1),
            float(j2),
            float(j3),
            float(j4),
            float(j5),
            float(j6)
        ]

        if not self._arm_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The UR3 action server is not available'
            )
            return

        self.get_logger().info(
            f'Sending joint goal: '
            f'[{j1}, {j2}, {j3}, {j4}, {j5}, {j6}]'
        )

        future = self._arm_action_client.send_goal_async(
            goal_msg
        )

        future.add_done_callback(
            self.arm_goal_response_callback
        )

    def attach_and_lift(
            self, z_offset, object_x=0.62, object_y=0.0, object_z=0.05):
        """Attach pick_box to the gripper, then lift it along base_link Z."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.ATTACH_AND_LIFT
        goal_msg.cartesian_z_offset = float(z_offset)
        goal_msg.object_x = float(object_x)
        goal_msg.object_y = float(object_y)
        goal_msg.object_z = float(object_z)

        if not self._arm_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The UR3 action server is not available'
            )
            return

        self.get_logger().info(
            f'Attaching pick_box and requesting Cartesian lift: '
            f'object=({object_x:.3f}, {object_y:.3f}, {object_z:.3f}), '
            f'dz={z_offset:+.3f} m'
        )

        future = self._arm_action_client.send_goal_async(
            goal_msg
        )

        future.add_done_callback(
            self.arm_goal_response_callback
        )

    def grasp_fixed_bottle(
            self,
            approach_clearance=FIXED_APPROACH_CLEARANCE,
            tcp_grasp_offset=FIXED_TCP_GRASP_OFFSET,
            grasp_depth_offset=FIXED_GRASP_DEPTH_OFFSET,
            lift_offset=FIXED_LIFT_OFFSET,
            open_position=FIXED_OPEN_POSITION,
            close_position=FIXED_CLOSE_POSITION,
            max_effort=FIXED_MAX_EFFORT):
        """Side-grasp the fixed bottle and stop after lifting it."""

        object_x = FIXED_BOTTLE_X
        object_y = FIXED_BOTTLE_Y
        object_z = FIXED_BOTTLE_Z

        grasp_z = float(object_z) + float(tcp_grasp_offset)
        geometry = radial_side_grasp_geometry(
            float(object_x),
            float(object_y),
            float(approach_clearance),
            float(grasp_depth_offset),
        )
        transport_side = random.choice((-1.0, 1.0))
        transport_dx = random.uniform(
            -FIXED_TRANSPORT_X_MAX,
            FIXED_TRANSPORT_X_MAX,
        )
        transport_dy = transport_side * random.uniform(
            FIXED_TRANSPORT_Y_MIN,
            FIXED_TRANSPORT_Y_MAX,
        )
        self._pick_sequence = {
            'stage': 'opening',
            'object_x': float(object_x),
            'object_y': float(object_y),
            'object_z': float(object_z),
            'grasp_x': geometry['grasp_x'],
            'grasp_y': geometry['grasp_y'],
            'grasp_z': grasp_z,
            'approach_x': geometry['approach_x'],
            'approach_y': geometry['approach_y'],
            'advance_x': geometry['advance_x'],
            'advance_y': geometry['advance_y'],
            'grasp_quaternion': geometry['quaternion'],
            'transport_dx': transport_dx,
            'transport_dy': transport_dy,
            'lift_offset': float(lift_offset),
            'open_position': float(open_position),
            'close_position': float(close_position),
            'max_effort': float(max_effort),
        }

        self.get_logger().info(
            'Starting fixed-bottle side-grasp: '
            f'object=({object_x:.3f}, {object_y:.3f}, {object_z:.3f}), '
            f'pre_grasp=({geometry["approach_x"]:.3f}, '
            f'{geometry["approach_y"]:.3f}, {grasp_z:.3f}), '
            f'grasp_tcp=({geometry["grasp_x"]:.3f}, '
            f'{geometry["grasp_y"]:.3f}, {grasp_z:.3f}), '
            f'approach_yaw={math.degrees(geometry["yaw"]):+.1f} deg, '
            f'post_lift_cartesian=(dx={transport_dx:+.3f}, '
            f'dy={transport_dy:+.3f}, dz=+0.000)'
        )
        self.open_gripper(open_position, max_effort)

    def abort_pick_sequence(self, reason):
        """Stop the combined sequence after a failed stage."""

        self.get_logger().error(f'Fixed-bottle grasp aborted: {reason}')
        self._pick_sequence = None
        rclpy.shutdown()

    def move_cartesian(self, x_offset, y_offset, z_offset, strict=False):
        """Move gripper_tcp by an XYZ offset in the base_link frame."""

        x_offset = float(x_offset)
        y_offset = float(y_offset)
        z_offset = float(z_offset)
        if max(abs(x_offset), abs(y_offset), abs(z_offset)) < 1e-6:
            self.get_logger().error('At least one Cartesian offset must be non-zero')
            return

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = (
            UR3Control.Goal.MOVE_CARTESIAN_STRICT
            if strict else UR3Control.Goal.MOVE_CARTESIAN
        )
        goal_msg.cartesian_x_offset = x_offset
        goal_msg.cartesian_y_offset = y_offset
        goal_msg.cartesian_z_offset = z_offset

        if not self._arm_action_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error('The UR3 action server is not available')
            return

        motion_kind = 'strict Cartesian' if strict else 'Cartesian'
        self.get_logger().info(
            f'Requesting {motion_kind} motion from current gripper pose: '
            f'dx={x_offset:+.3f}, dy={y_offset:+.3f}, '
            f'dz={z_offset:+.3f} m'
        )
        future = self._arm_action_client.send_goal_async(goal_msg)
        future.add_done_callback(self.arm_goal_response_callback)

    def move_to_xy(self, target_x, target_y):
        """Move to absolute base_link X/Y while keeping current Z and orientation."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.MOVE_TO_XY
        goal_msg.target_x = float(target_x)
        goal_msg.target_y = float(target_y)

        if not self._arm_action_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error('The UR3 action server is not available')
            return

        self.get_logger().info(
            f'Requesting absolute XY target in base_link: '
            f'x={target_x:+.3f}, y={target_y:+.3f}; keeping current Z'
        )
        future = self._arm_action_client.send_goal_async(goal_msg)
        future.add_done_callback(self.arm_goal_response_callback)

    def detach_object(self, open_position=0.06, max_effort=100.0):
        """Detach pick_box, then automatically open the gripper."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.DETACH_OBJECT

        if not self._arm_action_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error('The UR3 action server is not available')
            return

        self.get_logger().info(
            'Requesting pick_box detach; gripper will open afterward...'
        )
        future = self._arm_action_client.send_goal_async(goal_msg)
        future.add_done_callback(
            lambda response_future: self.detach_goal_response_callback(
                response_future,
                float(open_position),
                float(max_effort)
            )
        )

    def detach_goal_response_callback(
            self, future, open_position, max_effort):
        """Wait specifically for detach, then arrange automatic opening."""

        try:
            goal_handle = future.result()
        except Exception as error:
            self.get_logger().error(f'Failed to send detach goal: {error}')
            return

        if not goal_handle.accepted:
            self.get_logger().error('The detach goal was rejected')
            return

        self.get_logger().info('The detach goal was accepted')
        result_future = goal_handle.get_result_async()
        result_future.add_done_callback(
            lambda detach_future: self.detach_result_callback(
                detach_future,
                open_position,
                max_effort
            )
        )

    def detach_result_callback(self, future, open_position, max_effort):
        """Open the gripper only after the detach goal has completed."""

        try:
            wrapped_result = future.result()
            result = wrapped_result.result
        except Exception as error:
            self.get_logger().error(f'Failed to receive detach result: {error}')
            return

        self.get_logger().info(
            f'Detach action completed: success={result.success}, '
            f'message="{result.message}"'
        )
        if (
            self._pick_sequence is not None
            and self._pick_sequence['stage'] == 'detaching'
        ):
            self._pick_sequence['stage'] = 'releasing'
        self.get_logger().info(
            'Detach action finished; opening the gripper automatically'
        )
        self.open_gripper(open_position, max_effort)

    def arm_goal_response_callback(self, future):
        """Process the response after sending an arm goal."""

        try:
            goal_handle = future.result()
        except Exception as error:
            self.get_logger().error(
                f'Failed to send the arm goal: {error}'
            )
            return

        if not goal_handle.accepted:
            self.get_logger().error(
                'The arm goal was rejected'
            )
            return

        self.get_logger().info(
            'The arm goal was accepted'
        )

        result_future = goal_handle.get_result_async()

        result_future.add_done_callback(
            self.arm_result_callback
        )

    def arm_result_callback(self, future):
        """Process the final arm result."""

        try:
            wrapped_result = future.result()
            result = wrapped_result.result
        except Exception as error:
            self.get_logger().error(
                f'Failed to receive the arm result: {error}'
            )
            return

        self.get_logger().info(
            f'Arm action completed: '
            f'success={result.success}, '
            f'message="{result.message}"'
        )

        if self._pick_sequence is not None:
            if not result.success:
                self.abort_pick_sequence(
                    f'arm stage {self._pick_sequence["stage"]} failed'
                )
                return

            stage = self._pick_sequence['stage']
            if stage == 'pregrasp':
                self.capture_pregrasp_joints()
                self._pick_sequence['stage'] = 'advancing'
                self.get_logger().info(
                    'Fixed grasp stage 2/5: advancing horizontally to the bottle'
                )
                self.move_cartesian(
                    self._pick_sequence['advance_x'],
                    self._pick_sequence['advance_y'],
                    0.0,
                    strict=True)
                return

            if stage == 'advancing':
                self._pick_sequence['stage'] = 'closing'
                self.get_logger().info(
                    'Fixed grasp stage 3/5: closing both gripper fingers'
                )
                self.close_gripper(
                    self._pick_sequence['close_position'],
                    self._pick_sequence['max_effort'])
                return

            if stage == 'lifting':
                self._pick_sequence['stage'] = 'transporting'
                self.get_logger().info(
                    'Fixed grasp stage 5/5: moving the lifted bottle sideways '
                    'with a strict Cartesian path'
                )
                self.move_cartesian(
                    self._pick_sequence['transport_dx'],
                    self._pick_sequence['transport_dy'],
                    0.0,
                    strict=True)
                return

            if stage == 'transporting':
                self.get_logger().info(
                    'Fixed-bottle side-grasp and Cartesian transport '
                    'completed successfully'
                )
                self.save_successful_waypoint()
                self._pick_sequence = None
                rclpy.shutdown()
                return

        rclpy.shutdown()

    # =========================================================
    # GRIPPER COMMANDS
    # =========================================================

    def send_gripper_goal(self, position, max_effort):
        """
        Send a position goal to the gripper.

        Args:
            position:
                Desired gripper position.

            max_effort:
                Maximum allowed gripper effort.
        """

        self._latest_gripper_effort = None
        self._peak_gripper_effort = 0.0

        if not self._gripper_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The gripper action server is not available'
            )
            return

        requested_position = float(position)
        safe_position = min(
            max(requested_position, self.GRIPPER_MIN_POSITION),
            self.GRIPPER_MAX_POSITION
        )
        if safe_position != requested_position:
            self.get_logger().warning(
                f'Clamping gripper position from {requested_position:.4f} '
                f'to safe range value {safe_position:.4f}'
            )

        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = [
            'gripper_joint',
            'gripper_joint_mimic'
        ]
        point = JointTrajectoryPoint()
        point.positions = [safe_position, safe_position]
        point.time_from_start = Duration(sec=2, nanosec=0)
        goal_msg.trajectory.points = [point]

        self.get_logger().info(
            f'Sending gripper goal: '
            f'left={safe_position:.4f}, right={safe_position:.4f}, '
            f'requested_max_effort={max_effort:.2f}'
        )

        future = self._gripper_action_client.send_goal_async(
            goal_msg,
            feedback_callback=self.gripper_feedback_callback
        )

        future.add_done_callback(
            self.gripper_goal_response_callback
        )

    def open_gripper(self, position, max_effort):
        """
        Open the gripper using the supplied position and effort.
        """

        self.send_gripper_goal(
            position=position,
            max_effort=max_effort
        )

    def close_gripper(self, position, max_effort):
        """
        Close the gripper using the supplied position and effort.
        """

        self.send_gripper_goal(
            position=position,
            max_effort=max_effort
        )

    def grasp_and_lift(self, close_position, max_effort, z_offset):
        """Close the fingers, then physically attach and lift the box."""

        self._pending_lift_offset = float(z_offset)
        self.close_gripper(close_position, max_effort)

    def gripper_goal_response_callback(self, future):
        """Process the response after sending a gripper goal."""

        try:
            goal_handle = future.result()
        except Exception as error:
            self.get_logger().error(
                f'Failed to send the gripper goal: {error}'
            )
            return

        if not goal_handle.accepted:
            self.get_logger().error(
                'The gripper goal was rejected'
            )
            return

        self.get_logger().info(
            'The gripper goal was accepted'
        )

        # Keep a fallback for a simulated trajectory controller that does not
        # return after the commanded two-second finger trajectory.
        if self._pending_lift_offset is not None:
            self._gripper_completion_timer = self.create_timer(
                3.0,
                self.gripper_completion_timeout_callback
            )

        result_future = goal_handle.get_result_async()

        result_future.add_done_callback(
            self.gripper_result_callback
        )

    def request_pending_lift(self, reason):
        """Request attach/lift once after gripper completion or timeout."""

        if self._pending_lift_offset is None:
            return

        if self._gripper_completion_timer is not None:
            self._gripper_completion_timer.cancel()
            self.destroy_timer(self._gripper_completion_timer)
            self._gripper_completion_timer = None

        z_offset = self._pending_lift_offset
        self._pending_lift_offset = None
        self.get_logger().info(
            f'{reason}; requesting Gazebo attach and Cartesian lift'
        )
        self.attach_and_lift(z_offset)

    def gripper_completion_timeout_callback(self):
        """Continue a simulated grasp if the gripper action never finishes."""

        self.get_logger().warning(
            'Gripper action did not return a result within 3 seconds'
        )
        self.request_pending_lift('Gripper close timeout reached')

    def gripper_feedback_callback(self, feedback_msg):
        """Receive feedback while the gripper is moving."""

        feedback = feedback_msg.feedback
        actual_positions = list(feedback.actual.positions)
        desired_positions = list(feedback.desired.positions)

        self.get_logger().info(
            f'Gripper feedback: '
            f'desired_positions={desired_positions}, '
            f'actual_positions={actual_positions}, '
            f'measured_effort={self._latest_gripper_effort}, '
            f'peak_measured_effort={self._peak_gripper_effort:.2f}'
        )

    def gripper_result_callback(self, future):
        """Process the final gripper result."""

        try:
            wrapped_result = future.result()
            result = wrapped_result.result
            status = wrapped_result.status
        except Exception as error:
            self.get_logger().error(
                f'Failed to receive the gripper result: {error}'
            )
            return

        self.get_logger().info(
            f'Gripper action completed: '
            f'error_code={result.error_code}, '
            f'error_string="{result.error_string}", '
            f'measured_effort={self._latest_gripper_effort}, '
            f'peak_measured_effort={self._peak_gripper_effort:.2f}, '
            f'status={status}'
        )

        if self._pick_sequence is not None:
            if result.error_code != 0:
                self.abort_pick_sequence(
                    f'gripper stage {self._pick_sequence["stage"]} failed: '
                    f'{result.error_string}'
                )
                return

            stage = self._pick_sequence['stage']
            if stage == 'opening':
                self._pick_sequence['stage'] = 'pregrasp'
                self.get_logger().info(
                    'Fixed grasp stage 1/5: automatically planning to the '
                    'side pre-grasp pose'
                )
                qx, qy, qz, qw = self._pick_sequence['grasp_quaternion']
                self.send_pose_goal(
                    self._pick_sequence['approach_x'],
                    self._pick_sequence['approach_y'],
                    self._pick_sequence['grasp_z'],
                    qx, qy, qz, qw)
                return

            if stage == 'closing':
                self._pick_sequence['stage'] = 'lifting'
                self.get_logger().info(
                    'Fixed grasp stage 4/5: attaching and lifting the bottle'
                )
                self.attach_and_lift(
                    self._pick_sequence['lift_offset'],
                    self._pick_sequence['object_x'],
                    self._pick_sequence['object_y'],
                    self._pick_sequence['object_z'])
                return

            if stage == 'releasing':
                self.get_logger().info(
                    'Pick-and-move sequence completed successfully; '
                    'the object was released at the target'
                )
                self._pick_sequence = None
                rclpy.shutdown()
                return

        if self._pending_lift_offset is not None:
            self.request_pending_lift('Gripper action completed')
            return

        rclpy.shutdown()


def main(args=None):
    rclpy.init(args=args)

    action_client = UR3ActionClient()

    # Repeatable side-grasp test. All fixed coordinates use base_link, and the
    # program stops after lifting so the final grasp can be inspected.
    action_client.grasp_fixed_bottle()

    try:
        rclpy.spin(action_client)

    except KeyboardInterrupt:
        action_client.get_logger().info(
            'The program was stopped by the user'
        )

    finally:
        action_client.destroy_node()

        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()

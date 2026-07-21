#!/usr/bin/env python3

import time

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient

from control_msgs.action import GripperCommand
from sensor_msgs.msg import JointState
from std_msgs.msg import Float64MultiArray
from ur3_moveit_control.action import UR3Control


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
            GripperCommand,
            '/gripper_controller/gripper_cmd'
        )
        self._gripper_mimic_publisher = self.create_publisher(
            Float64MultiArray,
            '/gripper_mimic_controller/commands',
            10
        )
        self._pending_lift_offset = None
        self._gripper_completion_timer = None
        self._latest_gripper_effort = None
        self._peak_gripper_effort = 0.0
        self._joint_state_subscription = self.create_subscription(
            JointState,
            '/joint_states',
            self.joint_state_callback,
            10
        )

    def joint_state_callback(self, msg):
        """Track measured gripper effort reported by ros2_control."""

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

    def attach_and_lift(self, z_offset):
        """Attach pick_box to the gripper, then lift it along base_link Z."""

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.ATTACH_AND_LIFT
        goal_msg.cartesian_z_offset = float(z_offset)

        if not self._arm_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The UR3 action server is not available'
            )
            return

        self.get_logger().info(
            f'Attaching pick_box and requesting Cartesian lift: '
            f'dz={z_offset:+.3f} m'
        )

        future = self._arm_action_client.send_goal_async(
            goal_msg
        )

        future.add_done_callback(
            self.arm_goal_response_callback
        )

    def move_cartesian(self, x_offset, y_offset, z_offset):
        """Move gripper_tcp by an XYZ offset in the base_link frame."""

        x_offset = float(x_offset)
        y_offset = float(y_offset)
        z_offset = float(z_offset)
        if max(abs(x_offset), abs(y_offset), abs(z_offset)) < 1e-6:
            self.get_logger().error('At least one Cartesian offset must be non-zero')
            return

        goal_msg = UR3Control.Goal()
        goal_msg.command_type = UR3Control.Goal.MOVE_CARTESIAN
        goal_msg.cartesian_x_offset = x_offset
        goal_msg.cartesian_y_offset = y_offset
        goal_msg.cartesian_z_offset = z_offset

        if not self._arm_action_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error('The UR3 action server is not available')
            return

        self.get_logger().info(
            f'Requesting Cartesian motion from current gripper pose: '
            f'dx={x_offset:+.3f}, dy={y_offset:+.3f}, '
            f'dz={z_offset:+.3f} m'
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

        goal_msg = GripperCommand.Goal()
        goal_msg.command.position = safe_position
        goal_msg.command.max_effort = float(max_effort)

        # Fortress does not reliably enforce URDF mimic joints through
        # ign_ros2_control. Command the second finger explicitly with the same
        # joint displacement; its rotated joint frame makes it move opposite
        # to the first finger in physical space.
        discovery_deadline = time.monotonic() + 2.0
        while (
            self._gripper_mimic_publisher.get_subscription_count() == 0
            and time.monotonic() < discovery_deadline
        ):
            time.sleep(0.05)

        if self._gripper_mimic_publisher.get_subscription_count() == 0:
            self.get_logger().error(
                'gripper_mimic_controller is not active; refusing to move '
                'only one finger'
            )
            return

        mimic_command = Float64MultiArray()
        mimic_command.data = [safe_position]
        # Publish more than once to make startup behavior deterministic even
        # when DDS endpoint discovery has only just completed.
        for _ in range(3):
            self._gripper_mimic_publisher.publish(mimic_command)
            time.sleep(0.02)

        self.get_logger().info(
            f'Sending gripper goal: '
            f'position={safe_position:.4f}, '
            f'max_effort={max_effort:.2f}'
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

        # Some simulated GripperActionController versions keep the goal active
        # after the fingers have physically stopped. Do not let that prevent
        # the grasp sequence from reaching the Gazebo attach step.
        if self._pending_lift_offset is not None:
            self._gripper_completion_timer = self.create_timer(
                2.0,
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
            'Gripper action did not return a result within 2 seconds'
        )
        self.request_pending_lift('Gripper close timeout reached')

    def gripper_feedback_callback(self, feedback_msg):
        """Receive feedback while the gripper is moving."""

        feedback = feedback_msg.feedback

        self.get_logger().info(
            f'Gripper feedback: '
            f'position={feedback.position:.6f}, '
            f'action_effort={feedback.effort:.2f}, '
            f'measured_effort={self._latest_gripper_effort}, '
            f'peak_measured_effort={self._peak_gripper_effort:.2f}, '
            f'stalled={feedback.stalled}, '
            f'reached_goal={feedback.reached_goal}'
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
            f'position={result.position:.6f}, '
            f'action_effort={result.effort:.2f}, '
            f'measured_effort={self._latest_gripper_effort}, '
            f'peak_measured_effort={self._peak_gripper_effort:.2f}, '
            f'stalled={result.stalled}, '
            f'reached_goal={result.reached_goal}, '
            f'status={status}'
        )

        if self._pending_lift_offset is not None:
            self.request_pending_lift('Gripper action completed')
            return

        rclpy.shutdown()


def main(args=None):
    rclpy.init(args=args)

    action_client = UR3ActionClient()

    # Select only one command for each test.

    # Move the robot to the home configuration
    # action_client.send_home_goal()

    # Send a Cartesian pose goal
    # action_client.send_pose_goal(0.34,0.0,0.22,1.0,0.0,0.0,0.0)

    # Move an attached object in base_link coordinates (x, y, z):
    # action_client.move_cartesian(0.1,0.1,0.0)

    # Send a joint position goal
    # action_client.send_joint_goal(-1.57,-1.57,1.57,-1.57,-1.57,0.0)

    # Open the gripper
    # action_client.open_gripper(0.06,10.0)

    # Close the gripper
    # action_client.close_gripper(0.0,100.0)

    # Close the gripper, attach the box in Gazebo + MoveIt, then lift 15 cm.
    # Run this only after the gripper TCP has been moved around the box.
    # action_client.grasp_and_lift(0.04, 100.0, 0.1)

    # After moving the attached object to its destination, select this command
    # instead of attach_and_lift() to release it:
    action_client.detach_object()


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

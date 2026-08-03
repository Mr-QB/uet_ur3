#!/usr/bin/env python3

import argparse
import csv
import math
from pathlib import Path
import random
import shutil
import subprocess
import sys
import time

import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node
from rclpy.utilities import remove_ros_args

from builtin_interfaces.msg import Duration
from control_msgs.action import FollowJointTrajectory
from sensor_msgs.msg import JointState
from trajectory_msgs.msg import JointTrajectoryPoint
from ur3_moveit_control.action import UR3Control
from grasp_profile import (
    APPROACH_CLEARANCE,
    CLOSE_POSITION,
    GRASP_DEPTH_OFFSET,
    LIFT_OFFSET,
    OPEN_POSITION,
    POUR_WRIST_ANGLE_DEG,
    TCP_GRASP_OFFSET,
    TRANSPORT_X_MAX,
    TRANSPORT_Y_MAX,
    TRANSPORT_Y_MIN,
    pouring_joint_goal,
    radial_side_grasp_geometry,
)


ARM_JOINT_NAMES = [
    'shoulder_pan_joint',
    'shoulder_lift_joint',
    'elbow_joint',
    'wrist_1_joint',
    'wrist_2_joint',
    'wrist_3_joint',
]


class RandomPickTester(Node):
    """Run repeatable pick-pour trials with one reusable Gazebo bottle."""

    def __init__(self, args):
        super().__init__('ur3_random_pick_test')
        self.args = args
        self._rng = random.Random(args.seed)
        self._arm_client = ActionClient(self, UR3Control, 'ur3_control')
        self._gripper_client = ActionClient(
            self,
            FollowJointTrajectory,
            '/gripper_controller/follow_joint_trajectory',
        )
        self._transport_cli = shutil.which('ign') or shutil.which('gz')
        self._success_count = 0
        self._latest_joint_positions = {}
        self._captured_pregrasp_joints = None
        self._joint_state_subscription = self.create_subscription(
            JointState,
            '/joint_states',
            self.joint_state_callback,
            10,
        )

    def joint_state_callback(self, msg):
        self._latest_joint_positions = dict(zip(msg.name, msg.position))

    def capture_pregrasp_joints(self):
        if any(
            name not in self._latest_joint_positions
            for name in ARM_JOINT_NAMES
        ):
            self._captured_pregrasp_joints = None
            self.get_logger().warning(
                'Pre-grasp succeeded, but the six arm joint states were not '
                'available for capture'
            )
            return
        self._captured_pregrasp_joints = [
            self._latest_joint_positions[name] for name in ARM_JOINT_NAMES
        ]
        self.get_logger().info(
            'Captured random-trial pre-grasp joints: ['
            + ', '.join(
                f'{value:.9f}' for value in self._captured_pregrasp_joints
            )
            + ']'
        )

    def wait_for_servers(self):
        if not self._arm_client.wait_for_server(timeout_sec=10.0):
            self.get_logger().error('The UR3 action server is not available')
            return False
        if not self._gripper_client.wait_for_server(timeout_sec=10.0):
            self.get_logger().error(
                'The gripper trajectory action server is not available'
            )
            return False
        if self._transport_cli is None:
            self.get_logger().error(
                'Neither "ign" nor "gz" was found; cannot reset pick_box pose'
            )
            return False
        return True

    def wait_for_future(self, future, description):
        rclpy.spin_until_future_complete(
            self,
            future,
            timeout_sec=self.args.action_timeout,
        )
        if not future.done():
            self.get_logger().error(f'Timeout while waiting for {description}')
            return None
        try:
            return future.result()
        except Exception as error:
            self.get_logger().error(f'{description} raised: {error}')
            return None

    def send_arm_goal(self, goal, stage):
        send_future = self._arm_client.send_goal_async(goal)
        goal_handle = self.wait_for_future(
            send_future,
            f'{stage} goal response',
        )
        if goal_handle is None or not goal_handle.accepted:
            return False, f'{stage}: goal rejected'

        result_future = goal_handle.get_result_async()
        wrapped_result = self.wait_for_future(
            result_future,
            f'{stage} result',
        )
        if wrapped_result is None:
            return False, f'{stage}: no action result'

        result = wrapped_result.result
        if not result.success:
            return False, f'{stage}: {result.message}'
        return True, result.message

    def send_gripper_goal(self, position, stage):
        goal = FollowJointTrajectory.Goal()
        goal.trajectory.joint_names = [
            'gripper_joint',
            'gripper_joint_mimic',
        ]
        point = JointTrajectoryPoint()
        point.positions = [float(position), float(position)]
        point.time_from_start = Duration(sec=2)
        goal.trajectory.points = [point]

        send_future = self._gripper_client.send_goal_async(goal)
        goal_handle = self.wait_for_future(
            send_future,
            f'{stage} gripper goal response',
        )
        if goal_handle is None or not goal_handle.accepted:
            return False, f'{stage}: gripper goal rejected'

        result_future = goal_handle.get_result_async()
        wrapped_result = self.wait_for_future(
            result_future,
            f'{stage} gripper result',
        )
        if wrapped_result is None:
            return False, f'{stage}: no gripper result'

        result = wrapped_result.result
        if result.error_code != 0:
            return False, f'{stage}: {result.error_string}'
        return True, result.error_string

    def prepare_next_trial(self):
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.PREPARE_NEXT_TRIAL
        return self.send_arm_goal(goal, 'prepare_next_trial')

    def move_home(self):
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.MOVE_HOME
        return self.send_arm_goal(goal, 'home')

    def move_to_pose(self, x, y, z, quaternion):
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.MOVE_POSE
        goal.pose_goal.header.frame_id = 'base_link'
        goal.pose_goal.header.stamp = self.get_clock().now().to_msg()
        goal.pose_goal.pose.position.x = float(x)
        goal.pose_goal.pose.position.y = float(y)
        goal.pose_goal.pose.position.z = float(z)
        qx, qy, qz, qw = quaternion
        goal.pose_goal.pose.orientation.x = qx
        goal.pose_goal.pose.orientation.y = qy
        goal.pose_goal.pose.orientation.z = qz
        goal.pose_goal.pose.orientation.w = qw
        result = self.send_arm_goal(goal, 'approach')
        if result[0]:
            self.capture_pregrasp_joints()
        return result

    def move_cartesian(self, dx, dy, dz, stage, strict=False):
        goal = UR3Control.Goal()
        goal.command_type = (
            UR3Control.Goal.MOVE_CARTESIAN_STRICT
            if strict else UR3Control.Goal.MOVE_CARTESIAN
        )
        goal.cartesian_x_offset = float(dx)
        goal.cartesian_y_offset = float(dy)
        goal.cartesian_z_offset = float(dz)
        return self.send_arm_goal(goal, stage)

    def attach_and_lift(self, object_x, object_y, object_z):
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.ATTACH_AND_LIFT
        goal.object_x = float(object_x)
        goal.object_y = float(object_y)
        goal.object_z = float(object_z)
        goal.cartesian_z_offset = float(self.args.lift_offset)
        return self.send_arm_goal(goal, 'attach_and_lift')

    def detach(self):
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.DETACH_OBJECT
        return self.send_arm_goal(goal, 'detach')

    def rotate_gripper_for_pour(self):
        """Plan a wrist-only tilt while keeping the carried bottle attached."""

        missing = [
            name for name in ARM_JOINT_NAMES
            if name not in self._latest_joint_positions
        ]
        if missing:
            return False, (
                'pour_wrist: missing current joint states: '
                + ', '.join(missing)
            )

        current_joints = [
            self._latest_joint_positions[name] for name in ARM_JOINT_NAMES
        ]
        try:
            target_joints, applied_delta = pouring_joint_goal(
                current_joints,
                self.args.pour_angle_deg,
            )
        except ValueError as error:
            return False, f'pour_wrist: {error}'

        self.get_logger().info(
            'Planning bottle-pouring tilt using wrist_3_joint only: '
            f'requested={self.args.pour_angle_deg:+.1f} deg, '
            f'applied={math.degrees(applied_delta):+.1f} deg, '
            f'target={target_joints[5]:+.3f} rad'
        )
        goal = UR3Control.Goal()
        goal.command_type = UR3Control.Goal.MOVE_JOINT
        goal.joint_goal.name = list(ARM_JOINT_NAMES)
        goal.joint_goal.position = target_joints
        return self.send_arm_goal(goal, 'pour_wrist')

    def hide_carried_bottle(self):
        """Detach internally, park the bottle, and clear its MoveIt object."""

        detached, message = self.detach()
        if not detached:
            return False, f'hide_bottle detach failed: {message}'

        parked, message = self.park_gazebo_box()
        if not parked:
            return False, f'hide_bottle parking failed: {message}'

        prepared, message = self.prepare_next_trial()
        if not prepared:
            return False, f'hide_bottle scene cleanup failed: {message}'

        return True, (
            'Bottle detached internally and moved directly to parking; '
            'the gripper remained closed'
        )

    def gazebo_box_exists(self):
        """Confirm that Gazebo currently contains pick_box::box_link."""

        command = [
            self._transport_cli,
            'model',
            '-m',
            'pick_box',
            '-l',
            'box_link',
        ]
        diagnostics = ''
        for attempt in range(1, 4):
            try:
                completed = subprocess.run(
                    command,
                    capture_output=True,
                    check=False,
                    text=True,
                    timeout=self.args.gazebo_timeout + 2.0,
                )
                diagnostics = (
                    f'{completed.stdout}\n{completed.stderr}'
                ).strip()
                if (
                    completed.returncode == 0
                    and 'Name: box_link' in diagnostics
                ):
                    return True, diagnostics
            except (OSError, subprocess.TimeoutExpired) as error:
                diagnostics = str(error)

            if attempt < 3:
                time.sleep(0.2)

        return False, diagnostics or 'pick_box::box_link was not found'

    def set_gazebo_box_world_pose(self, world_x, world_y, world_z, label):
        exists, diagnostics = self.gazebo_box_exists()
        if not exists:
            return False, (
                'pick_box::box_link is missing. Random testing deliberately '
                'does not respawn it because DetachableJoint is bound to the '
                'original Gazebo entity ID. Restart the Gazebo launch so the '
                f'bottle is created before the robot plugin. Details: {diagnostics}'
            )

        service = f'/world/{self.args.world}/set_pose'

        using_ign = Path(self._transport_cli).name == 'ign'
        request_type = (
            'ignition.msgs.Pose' if using_ign else 'gz.msgs.Pose'
        )
        response_type = (
            'ignition.msgs.Boolean' if using_ign else 'gz.msgs.Boolean'
        )
        request = (
            'name: "pick_box", '
            f'position: {{x: {world_x:.6f}, y: {world_y:.6f}, '
            f'z: {world_z:.6f}}}, '
            'orientation: {x: 0.0, y: 0.0, z: 0.0, w: 1.0}'
        )
        command = [
            self._transport_cli,
            'service',
            '-s',
            service,
            '--reqtype',
            request_type,
            '--reptype',
            response_type,
            '--timeout',
            str(int(self.args.gazebo_timeout * 1000)),
            '--req',
            request,
        ]

        try:
            completed = subprocess.run(
                command,
                capture_output=True,
                check=False,
                text=True,
                timeout=self.args.gazebo_timeout + 2.0,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            return False, f'Gazebo set_pose failed: {error}'

        output = f'{completed.stdout}\n{completed.stderr}'.strip()
        if completed.returncode != 0:
            return False, (
                f'Gazebo set_pose exited with {completed.returncode}: {output}'
            )
        if 'data: false' in output.lower():
            return False, f'Gazebo rejected set_pose: {output}'

        # UserCommands acknowledges a queued set_pose with data=true even when
        # the named entity does not exist. Check the entity graph explicitly
        # so the trial cannot continue to close/attach around an absent bottle.
        exists, diagnostics = self.gazebo_box_exists()
        if not exists:
            return False, (
                'Gazebo acknowledged set_pose, but pick_box::box_link is '
                f'absent: {diagnostics}'
            )

        self.get_logger().info(
            f'Teleported pick_box to {label}: '
            f'world=({world_x:.3f}, {world_y:.3f}, {world_z:.3f})'
        )
        time.sleep(self.args.physics_settle_time)
        return True, output or 'Gazebo set_pose succeeded'

    def set_gazebo_box_pose(self, base_x, base_y, base_z):
        world_x = self.args.base_world_x + base_x
        world_y = self.args.base_world_y + base_y
        world_z = self.args.base_world_z + base_z
        self.get_logger().info(
            'Placing pick_box for trial: '
            f'base_link=({base_x:.3f}, {base_y:.3f}, {base_z:.3f})'
        )
        return self.set_gazebo_box_world_pose(
            world_x,
            world_y,
            world_z,
            'trial position',
        )

    def park_gazebo_box(self):
        return self.set_gazebo_box_world_pose(
            self.args.parking_world_x,
            self.args.parking_world_y,
            self.args.parking_world_z,
            'parking position',
        )

    def random_trial_coordinates(self):
        object_x = self._rng.uniform(
            self.args.pick_x_min,
            self.args.pick_x_max,
        )
        object_y = self._rng.uniform(
            self.args.pick_y_min,
            self.args.pick_y_max,
        )
        transport_dx = self._rng.uniform(
            -self.args.transport_x_max,
            self.args.transport_x_max,
        )
        side = self._rng.choice((-1.0, 1.0))
        transport_dy = side * self._rng.uniform(
            self.args.transport_y_min,
            self.args.transport_y_max,
        )
        return object_x, object_y, transport_dx, transport_dy

    def run_step(self, trial_number, stage, operation):
        self.get_logger().info(
            f'Trial {trial_number}/{self.args.trials}: {stage}'
        )
        success, message = operation()
        if not success:
            self.get_logger().error(
                f'Trial {trial_number} failed at {stage}: {message}'
            )
        return success, message

    def run_trial(
            self,
            trial_number,
            object_x,
            object_y,
            transport_dx,
            transport_dy):
        self._captured_pregrasp_joints = None
        object_z = self.args.object_z
        grasp_z = object_z + self.args.tcp_grasp_offset
        geometry = radial_side_grasp_geometry(
            object_x,
            object_y,
            self.args.approach_clearance,
            self.args.grasp_depth_offset,
        )
        self.get_logger().info(
            f'Trial {trial_number} grasp geometry: '
            f'approach=({geometry["approach_x"]:.3f}, '
            f'{geometry["approach_y"]:.3f}, {grasp_z:.3f}), '
            f'grasp=({geometry["grasp_x"]:.3f}, '
            f'{geometry["grasp_y"]:.3f}, {grasp_z:.3f}), '
            f'yaw={math.degrees(geometry["yaw"]):+.1f} deg'
        )

        steps = [
            ('prepare_next_trial', self.prepare_next_trial),
            ('park_gazebo_box', self.park_gazebo_box),
            ('home', self.move_home),
            (
                'set_gazebo_pose',
                lambda: self.set_gazebo_box_pose(
                    object_x,
                    object_y,
                    object_z,
                ),
            ),
            (
                'open_gripper',
                lambda: self.send_gripper_goal(
                    self.args.open_position,
                    'open_gripper',
                ),
            ),
            (
                'approach',
                lambda: self.move_to_pose(
                    geometry['approach_x'],
                    geometry['approach_y'],
                    grasp_z,
                    geometry['quaternion'],
                ),
            ),
            (
                'advance_to_grasp',
                lambda: self.move_cartesian(
                    geometry['advance_x'],
                    geometry['advance_y'],
                    0.0,
                    'advance_to_grasp',
                    strict=True,
                ),
            ),
            (
                'close_gripper',
                lambda: self.send_gripper_goal(
                    self.args.close_position,
                    'close_gripper',
                ),
            ),
            (
                'attach_and_lift',
                lambda: self.attach_and_lift(
                    object_x,
                    object_y,
                    object_z,
                ),
            ),
            (
                'transport_cartesian',
                lambda: self.move_cartesian(
                    transport_dx,
                    transport_dy,
                    0.0,
                    'transport_cartesian',
                    strict=True,
                ),
            ),
            ('pour_wrist', self.rotate_gripper_for_pour),
            ('hide_carried_bottle', self.hide_carried_bottle),
        ]

        for stage, operation in steps:
            success, message = self.run_step(
                trial_number,
                stage,
                operation,
            )
            if not success:
                return False, stage, message
        self.save_successful_waypoint(
            trial_number,
            object_x,
            object_y,
            object_z,
            grasp_z,
            geometry,
            transport_dx,
            transport_dy,
        )
        return True, 'completed', 'Pick, transport, pour, and cleanup completed'

    def append_result(self, result):
        result_path = Path(self.args.results_file).expanduser().resolve()
        result_path.parent.mkdir(parents=True, exist_ok=True)
        write_header = not result_path.exists()
        with result_path.open('a', newline='', encoding='utf-8') as csv_file:
            writer = csv.DictWriter(csv_file, fieldnames=result.keys())
            if write_header:
                writer.writeheader()
            writer.writerow(result)

    def save_successful_waypoint(
            self,
            trial_number,
            object_x,
            object_y,
            object_z,
            grasp_z,
            geometry,
            transport_dx,
            transport_dy):
        if self._captured_pregrasp_joints is None:
            self.get_logger().warning(
                'Trial succeeded, but no pre-grasp waypoint was captured'
            )
            return

        joint_names = [
            'shoulder_pan_joint',
            'shoulder_lift_joint',
            'elbow_joint',
            'wrist_1_joint',
            'wrist_2_joint',
            'wrist_3_joint',
        ]
        qx, qy, qz, qw = geometry['quaternion']
        row = {
            'trial': trial_number,
            'seed': self.args.seed,
            'object_x': object_x,
            'object_y': object_y,
            'object_z': object_z,
            'pregrasp_x': geometry['approach_x'],
            'pregrasp_y': geometry['approach_y'],
            'pregrasp_z': grasp_z,
            'qx': qx,
            'qy': qy,
            'qz': qz,
            'qw': qw,
            **dict(zip(joint_names, self._captured_pregrasp_joints)),
            'transport_dx': transport_dx,
            'transport_dy': transport_dy,
        }
        path = Path(self.args.waypoints_file).expanduser().resolve()
        write_header = not path.exists() or path.stat().st_size == 0
        with path.open('a', newline='', encoding='utf-8') as csv_file:
            writer = csv.DictWriter(csv_file, fieldnames=row.keys())
            if write_header:
                writer.writeheader()
            writer.writerow(row)
        self.get_logger().info(f'Saved successful waypoint to {path}')

    def run(self):
        if not self.wait_for_servers():
            return 1

        self.get_logger().info(
            f'Starting {self.args.trials} random trials with seed '
            f'{self.args.seed}; results={self.args.results_file}'
        )

        for trial_number in range(1, self.args.trials + 1):
            if not rclpy.ok():
                break

            object_x, object_y, transport_dx, transport_dy = (
                self.random_trial_coordinates()
            )
            self.get_logger().info(
                f'Trial {trial_number}: '
                f'pick=({object_x:.3f}, {object_y:.3f}), '
                f'carried_cartesian=(dx={transport_dx:+.3f}, '
                f'dy={transport_dy:+.3f}, dz=+0.000)'
            )

            start_time = time.monotonic()
            success, failed_stage, message = self.run_trial(
                trial_number,
                object_x,
                object_y,
                transport_dx,
                transport_dy,
            )
            elapsed = time.monotonic() - start_time
            if success:
                self._success_count += 1

            self.append_result({
                'trial': trial_number,
                'seed': self.args.seed,
                'object_x': f'{object_x:.6f}',
                'object_y': f'{object_y:.6f}',
                'object_z': f'{self.args.object_z:.6f}',
                'transport_dx': f'{transport_dx:.6f}',
                'transport_dy': f'{transport_dy:.6f}',
                'success': success,
                'failed_stage': '' if success else failed_stage,
                'message': message,
                'elapsed_wall_seconds': f'{elapsed:.3f}',
            })

        attempted = self.args.trials
        self.get_logger().info(
            f'Random test finished: {self._success_count}/{attempted} '
            f'trials succeeded; results={self.args.results_file}'
        )
        return 0 if self._success_count == attempted else 2


def parse_args(argv):
    parser = argparse.ArgumentParser(
        description='Repeated random pick-pour test using one pick_box.',
    )
    parser.add_argument('--trials', type=int, default=10)
    parser.add_argument('--seed', type=int, default=23)
    parser.add_argument('--world', default='empty')
    parser.add_argument('--results-file', default='random_pick_results.csv')
    parser.add_argument(
        '--waypoints-file',
        default='successful_random_waypoints.csv',
    )

    parser.add_argument('--pick-x-min', type=float, default=0.58)
    parser.add_argument('--pick-x-max', type=float, default=0.64)
    parser.add_argument('--pick-y-min', type=float, default=-0.06)
    parser.add_argument('--pick-y-max', type=float, default=0.06)
    parser.add_argument(
        '--transport-x-max', type=float, default=TRANSPORT_X_MAX)
    parser.add_argument(
        '--transport-y-min', type=float, default=TRANSPORT_Y_MIN)
    parser.add_argument(
        '--transport-y-max', type=float, default=TRANSPORT_Y_MAX)

    parser.add_argument('--base-world-x', type=float, default=0.0)
    parser.add_argument('--base-world-y', type=float, default=0.0)
    parser.add_argument('--base-world-z', type=float, default=0.775)
    parser.add_argument('--object-z', type=float, default=0.05)
    parser.add_argument('--parking-world-x', type=float, default=1.50)
    parser.add_argument('--parking-world-y', type=float, default=1.50)
    parser.add_argument('--parking-world-z', type=float, default=0.10)

    # Defaults come from the exact same profile as send_goal_client.py.
    parser.add_argument(
        '--approach-clearance', type=float, default=APPROACH_CLEARANCE)
    parser.add_argument(
        '--tcp-grasp-offset', type=float, default=TCP_GRASP_OFFSET)
    parser.add_argument(
        '--grasp-depth-offset', type=float, default=GRASP_DEPTH_OFFSET)
    parser.add_argument('--lift-offset', type=float, default=LIFT_OFFSET)
    parser.add_argument('--open-position', type=float, default=OPEN_POSITION)
    parser.add_argument('--close-position', type=float, default=CLOSE_POSITION)
    parser.add_argument(
        '--pour-angle-deg', type=float, default=POUR_WRIST_ANGLE_DEG,
        help=(
            'Signed wrist_3 rotation after transport; change the sign to '
            'reverse the pouring direction.'
        ))

    parser.add_argument('--action-timeout', type=float, default=120.0)
    parser.add_argument('--gazebo-timeout', type=float, default=3.0)
    # With the default RTF 2 world, 0.25 wall seconds provides roughly the
    # same 0.5 simulation seconds of settling as the previous RTF 1 setup.
    parser.add_argument('--physics-settle-time', type=float, default=0.25)

    args = parser.parse_args(remove_ros_args(args=argv)[1:])
    if args.trials < 1:
        parser.error('--trials must be at least 1')
    if args.pick_x_min >= args.pick_x_max:
        parser.error('--pick-x-min must be smaller than --pick-x-max')
    if args.pick_y_min >= args.pick_y_max:
        parser.error('--pick-y-min must be smaller than --pick-y-max')
    if args.transport_x_max < 0.0:
        parser.error('--transport-x-max must be non-negative')
    if args.transport_y_min <= 0.0:
        parser.error('--transport-y-min must be positive')
    if args.transport_y_min > args.transport_y_max:
        parser.error(
            '--transport-y-min must not exceed --transport-y-max'
        )
    if args.grasp_depth_offset < 0.0:
        parser.error('--grasp-depth-offset must be non-negative')
    if not 1.0 <= abs(args.pour_angle_deg) <= 150.0:
        parser.error('--pour-angle-deg magnitude must be between 1 and 150')
    return args


def main(argv=None):
    argv = sys.argv if argv is None else argv
    args = parse_args(argv)
    rclpy.init(args=argv)
    node = RandomPickTester(args)

    try:
        return_code = node.run()
    except KeyboardInterrupt:
        node.get_logger().info('Random pick test stopped by the user')
        return_code = 130
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()
    return return_code


if __name__ == '__main__':
    raise SystemExit(main())

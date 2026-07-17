#!/usr/bin/env python3

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient

from control_msgs.action import GripperCommand
from ur3_moveit_control.action import UR3Control


class UR3ActionClient(Node):

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

        if not self._gripper_action_client.wait_for_server(
            timeout_sec=5.0
        ):
            self.get_logger().error(
                'The gripper action server is not available'
            )
            return

        goal_msg = GripperCommand.Goal()
        goal_msg.command.position = float(position)
        goal_msg.command.max_effort = float(max_effort)

        self.get_logger().info(
            f'Sending gripper goal: '
            f'position={position:.4f}, '
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

        result_future = goal_handle.get_result_async()

        result_future.add_done_callback(
            self.gripper_result_callback
        )

    def gripper_feedback_callback(self, feedback_msg):
        """Receive feedback while the gripper is moving."""

        feedback = feedback_msg.feedback

        self.get_logger().info(
            f'Gripper feedback: '
            f'position={feedback.position:.6f}, '
            f'effort={feedback.effort:.2f}, '
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
            f'effort={result.effort:.2f}, '
            f'stalled={result.stalled}, '
            f'reached_goal={result.reached_goal}, '
            f'status={status}'
        )

        rclpy.shutdown()


def main(args=None):
    rclpy.init(args=args)

    action_client = UR3ActionClient()

    # Select only one command for each test.

    # Move the robot to the home configuration
    # action_client.send_home_goal()

    # Send a Cartesian pose goal
    action_client.send_pose_goal(0.35,0.0,0.35,0.0,0.0,0.0,1.0)

    # Send a joint position goal
    # action_client.send_joint_goal(-1.57,-1.57,1.57,-1.57,-1.57,0.0)

    # Open the gripper
    action_client.open_gripper(0.03,30.0)

    # Close the gripper
    # action_client.close_gripper(0.0,3.0)

    # Send a custom gripper command
    # action_client.send_gripper_goal(0.02,5.0)

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
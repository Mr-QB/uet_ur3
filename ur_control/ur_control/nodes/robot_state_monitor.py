import rclpy
from rclpy.node import Node
from sensor_msgs.msg import JointState


class RobotStateMonitor(Node):
    def __init__(self):
        super().__init__('robot_state_monitor')

        self.declare_parameter('joint_states_topic', '/joint_states')
        self.declare_parameter('print_period', 1.0)

        self.joint_states_topic = (
            self.get_parameter('joint_states_topic')
            .get_parameter_value()
            .string_value
        )

        self.print_period = (
            self.get_parameter('print_period')
            .get_parameter_value()
            .double_value
        )

        self.latest_joint_state = None

        self.joint_state_subscriber = self.create_subscription(
            JointState,
            self.joint_states_topic,
            self.joint_state_callback,
            10
        )

        self.timer = self.create_timer(
            self.print_period,
            self.timer_callback
        )

        self.get_logger().info(
            f'Robot State Monitor Node has started. Subscribed to {self.joint_states_topic}.'
        )

    def joint_state_callback(self, msg):
        self.latest_joint_state = msg

    def timer_callback(self):
        if self.latest_joint_state is None:
            self.get_logger().warn('Waiting for joint states to be published...')
            return

        names = self.latest_joint_state.name
        positions = self.latest_joint_state.position
        velocities = self.latest_joint_state.velocity
        efforts = self.latest_joint_state.effort

        self.get_logger().info('Current joint states:')

        for name, pos, vel, eff in zip(names, positions, velocities, efforts):
            self.get_logger().info(
                f'{name}: position={pos:.4f}, velocity={vel:.4f}, effort={eff:.4f}'
            )


def main(args=None):
    rclpy.init(args=args)

    node = RobotStateMonitor()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
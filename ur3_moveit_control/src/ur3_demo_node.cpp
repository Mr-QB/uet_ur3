#include <memory>
#include <thread>

#include <rclcpp/rclcpp.hpp>
#include <geometry_msgs/msg/pose.hpp>

#include "ur3_moveit_control/ur3_motion_interface.hpp"

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);

  auto node = rclcpp::Node::make_shared(
    "ur3_demo_node",
    rclcpp::NodeOptions().automatically_declare_parameters_from_overrides(true));

  rclcpp::executors::SingleThreadedExecutor executor;
  executor.add_node(node);

  std::thread spinner([&executor]() {
    executor.spin();
  });

  ur3_moveit_control::UR3MotionInterface ur3(node, "ur_manipulator");

  RCLCPP_INFO(node->get_logger(), "Moving UR3 to home position...");
  ur3.moveHome();

  std::vector<double> joint_goal = {
    -1.57,
    -1.20,
     1.50,
    -1.80,
    -1.57,
     0.00
  };

  RCLCPP_INFO(node->get_logger(), "Moving UR3 to joint target...");
  ur3.moveToJointGoal(joint_goal);

  geometry_msgs::msg::Pose target_pose;
  target_pose.position.x = 0.30;
  target_pose.position.y = 0.10;
  target_pose.position.z = 0.35;
  target_pose.orientation.x = 0.0;
  target_pose.orientation.y = 0.0;
  target_pose.orientation.z = 0.0;
  target_pose.orientation.w = 1.0;

  RCLCPP_INFO(node->get_logger(), "Trying pose target...");
  ur3.moveToPoseGoal(target_pose);

  executor.cancel();

  if (spinner.joinable()) {
    spinner.join();
  }

  rclcpp::shutdown();
  return 0;
}
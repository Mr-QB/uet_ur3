#include "ur3_moveit_control/ur3_motion_interface.hpp"

namespace ur3_moveit_control
{

UR3MotionInterface::UR3MotionInterface(
  const rclcpp::Node::SharedPtr & node,
  const std::string & planning_group)
: node_(node),
  move_group_(node_, planning_group)
{
  move_group_.setPlanningTime(5.0);
  move_group_.setNumPlanningAttempts(10);
  move_group_.setMaxVelocityScalingFactor(0.2);
  move_group_.setMaxAccelerationScalingFactor(0.2);

  RCLCPP_INFO(
    node_->get_logger(),
    "Planning group: %s",
    planning_group.c_str());

  RCLCPP_INFO(
    node_->get_logger(),
    "Planning frame: %s",
    move_group_.getPlanningFrame().c_str());

  RCLCPP_INFO(
    node_->get_logger(),
    "End-effector link: %s",
    move_group_.getEndEffectorLink().c_str());
}

bool UR3MotionInterface::moveToJointGoal(
  const std::vector<double> & joint_goal)
{
  if (joint_goal.size() != 6) {
    RCLCPP_ERROR(
      node_->get_logger(),
      "UR3 joint goal must contain exactly 6 joint values.");
    return false;
  }

  move_group_.setJointValueTarget(joint_goal);

  moveit::planning_interface::MoveGroupInterface::Plan plan;

  const bool planning_success =
    static_cast<bool>(move_group_.plan(plan));

  if (!planning_success) {
    RCLCPP_ERROR(node_->get_logger(), "Joint planning failed.");
    return false;
  }

  RCLCPP_INFO(node_->get_logger(), "Joint planning succeeded. Executing...");

  const auto execution_result = move_group_.execute(plan);

  if (execution_result != moveit::core::MoveItErrorCode::SUCCESS) {
    RCLCPP_ERROR(node_->get_logger(), "Joint execution failed.");
    return false;
  }

  RCLCPP_INFO(node_->get_logger(), "Joint execution succeeded.");
  return true;
}

bool UR3MotionInterface::moveToPoseGoal(
  const geometry_msgs::msg::Pose & target_pose,
  const std::string & end_effector_link)
{
  if (!end_effector_link.empty()) {
    move_group_.setEndEffectorLink(end_effector_link);
  }

  move_group_.setPoseTarget(target_pose);

  moveit::planning_interface::MoveGroupInterface::Plan plan;

  const bool planning_success =
    static_cast<bool>(move_group_.plan(plan));

  if (!planning_success) {
    RCLCPP_ERROR(node_->get_logger(), "Pose planning failed.");
    move_group_.clearPoseTargets();
    return false;
  }

  RCLCPP_INFO(node_->get_logger(), "Pose planning succeeded. Executing...");

  const auto execution_result = move_group_.execute(plan);

  move_group_.clearPoseTargets();

  if (execution_result != moveit::core::MoveItErrorCode::SUCCESS) {
    RCLCPP_ERROR(node_->get_logger(), "Pose execution failed.");
    return false;
  }

  RCLCPP_INFO(node_->get_logger(), "Pose execution succeeded.");
  return true;
}

bool UR3MotionInterface::moveHome()
{
  std::vector<double> home_joint_goal = {
    0.0,
    -1.57,
    1.57,
    -1.57,
    -1.57,
    0.0
  };

  return moveToJointGoal(home_joint_goal);
}

}  // namespace ur3_moveit_control
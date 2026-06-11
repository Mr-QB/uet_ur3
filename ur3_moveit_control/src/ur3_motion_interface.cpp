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


  double max_velocity_scaling = node_->get_parameter("max_velocity_scaling_factor").as_double();
  double max_acceleration_scaling = node_->get_parameter("max_acceleration_scaling_factor").as_double();
  double goal_pos_tol = node_->get_parameter("goal_position_tolerance").as_double();
  double goal_ori_tol = node_->get_parameter("goal_orientation_tolerance").as_double();
  double goal_joint_tol = node_->get_parameter("goal_joint_tolerance").as_double();

  move_group_.setMaxVelocityScalingFactor(max_velocity_scaling);
  move_group_.setMaxAccelerationScalingFactor(max_acceleration_scaling);

  // Set Planning Pipeline and Planner ID
  // move_group_.setPlanningPipelineId("ompl");
  move_group_.setPlannerId("RRTConnectkConfigDefault");

  // Set Goal Tolerances
  move_group_.setGoalPositionTolerance(goal_pos_tol);
  move_group_.setGoalOrientationTolerance(goal_ori_tol);
  move_group_.setGoalJointTolerance(goal_joint_tol);

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

  RCLCPP_INFO(
    node_->get_logger(),
    "Max velocity scaling factor: %.2f",
    max_velocity_scaling);

  RCLCPP_INFO(
    node_->get_logger(),
    "Max acceleration scaling factor: %.2f",
    max_acceleration_scaling);
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
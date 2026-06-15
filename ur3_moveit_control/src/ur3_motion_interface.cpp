#include "ur3_moveit_control/ur3_motion_interface.hpp"
#include <moveit_msgs/msg/collision_object.hpp>
#include <shape_msgs/msg/solid_primitive.hpp>

namespace ur3_moveit_control
{

  UR3MotionInterface::UR3MotionInterface(
      const rclcpp::Node::SharedPtr &node,
      const std::string &planning_group)
      : node_(node),
        move_group_(node_, planning_group)
  {
    move_group_.setPlanningTime(5.0);
    move_group_.setNumPlanningAttempts(10);

    double max_velocity_scaling = node_->get_parameter("max_velocity_scaling_factor").as_double();
    double max_acceleration_scaling = node_->get_parameter("max_acceleration_scaling_factor").as_double();
    std::string planner_id = node_->get_parameter("planner_id").as_string();
    double goal_pos_tol = node_->get_parameter("goal_position_tolerance").as_double();
    double goal_ori_tol = node_->get_parameter("goal_orientation_tolerance").as_double();
    double goal_joint_tol = node_->get_parameter("goal_joint_tolerance").as_double();

    move_group_.setMaxVelocityScalingFactor(max_velocity_scaling);
    move_group_.setMaxAccelerationScalingFactor(max_acceleration_scaling);

    // Set Planning Pipeline and Planner ID
    // move_group_.setPlanningPipelineId("ompl");
    move_group_.setPlannerId(planner_id);

    // Set Goal Tolerances
    move_group_.setGoalPositionTolerance(goal_pos_tol);
    move_group_.setGoalOrientationTolerance(goal_ori_tol);
    move_group_.setGoalJointTolerance(goal_joint_tol);

    RCLCPP_INFO(
        node_->get_logger(),
        "Planner ID: %s",
        planner_id.c_str());

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
      const std::vector<double> &joint_goal)
  {
    if (joint_goal.size() != 6)
    {
      RCLCPP_ERROR(
          node_->get_logger(),
          "UR3 joint goal must contain exactly 6 joint values.");
      return false;
    }

    move_group_.setJointValueTarget(joint_goal);

    moveit::planning_interface::MoveGroupInterface::Plan plan;

    const bool planning_success =
        static_cast<bool>(move_group_.plan(plan));

    if (!planning_success)
    {
      RCLCPP_ERROR(node_->get_logger(), "Joint planning failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Joint planning succeeded. Executing...");

    const auto execution_result = move_group_.execute(plan);

    if (execution_result != moveit::core::MoveItErrorCode::SUCCESS)
    {
      RCLCPP_ERROR(node_->get_logger(), "Joint execution failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Joint execution succeeded.");
    return true;
  }

  bool UR3MotionInterface::moveToPoseGoal(
      const geometry_msgs::msg::Pose &target_pose,
      const std::string &end_effector_link)
  {
    if (!end_effector_link.empty())
    {
      move_group_.setEndEffectorLink(end_effector_link);
    }

    move_group_.setPoseTarget(target_pose);

    moveit::planning_interface::MoveGroupInterface::Plan plan;

    const bool planning_success =
        static_cast<bool>(move_group_.plan(plan));

    if (!planning_success)
    {
      RCLCPP_ERROR(node_->get_logger(), "Pose planning failed.");
      move_group_.clearPoseTargets();
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Pose planning succeeded. Executing...");

    const auto execution_result = move_group_.execute(plan);

    move_group_.clearPoseTargets();

    if (execution_result != moveit::core::MoveItErrorCode::SUCCESS)
    {
      RCLCPP_ERROR(node_->get_logger(), "Pose execution failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Pose execution succeeded.");
    return true;
  }

  bool UR3MotionInterface::moveHome()
  {
    const bool target_set = move_group_.setNamedTarget("home");
    if (!target_set)
    {
      RCLCPP_ERROR(node_->get_logger(), "Failed to set named target: home");
      return false;
    }

    moveit::planning_interface::MoveGroupInterface::Plan plan;

    const bool planning_success =
        static_cast<bool>(move_group_.plan(plan));

    if (!planning_success)
    {
      RCLCPP_ERROR(node_->get_logger(), "Home planning failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Home planning succeeded. Executing...");

    const auto execution_result = move_group_.execute(plan);

    if (execution_result != moveit::core::MoveItErrorCode::SUCCESS)
    {
      RCLCPP_ERROR(node_->get_logger(), "Home execution failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Home execution succeeded.");
    return true;
  }

  void UR3MotionInterface::stop()
  {
    move_group_.stop();
  }

  void UR3MotionInterface::addTableObstacle()
  {
    moveit_msgs::msg::CollisionObject collision_object;
    collision_object.header.frame_id = move_group_.getPlanningFrame(); // usually "world" or "base_link"
    collision_object.id = "table";

    const double table_length = 1.2;
    const double table_width = 0.8;
    const double table_thickness = 0.02;
    const double wall_thickness = 0.02;
    const double wall_height = 0.8;
    const double back_wall_x = -0.2;
    const double side_wall_y = 0.5;
    const double front_wall_x = table_length / 2.0;
    const double side_wall_length = front_wall_x - back_wall_x;
    const double side_wall_x = (front_wall_x + back_wall_x) / 2.0;

    shape_msgs::msg::SolidPrimitive primitive;
    primitive.type = primitive.BOX;
    primitive.dimensions.resize(3);
    primitive.dimensions[0] = table_length;    // kích thước theo trục X
    primitive.dimensions[1] = table_width;     // kích thước theo trục Y
    primitive.dimensions[2] = table_thickness; // độ dày theo trục Z

    geometry_msgs::msg::Pose box_pose;
    box_pose.orientation.w = 1.0;
    box_pose.position.x = 0.0;
    box_pose.position.y = 0.0;
    box_pose.position.z = -table_thickness / 2.0;

    collision_object.primitives.push_back(primitive);
    collision_object.primitive_poses.push_back(box_pose);
    collision_object.operation = collision_object.ADD;

    shape_msgs::msg::SolidPrimitive back_wall_primitive;
    back_wall_primitive.type = back_wall_primitive.BOX;
    back_wall_primitive.dimensions.resize(3);
    back_wall_primitive.dimensions[0] = wall_thickness;      // độ dày theo trục X
    back_wall_primitive.dimensions[1] = side_wall_y * 2.0;   // hai bên cách gốc 0.5m
    back_wall_primitive.dimensions[2] = wall_height;         // chiều cao theo trục Z

    shape_msgs::msg::SolidPrimitive side_wall_primitive;
    side_wall_primitive.type = side_wall_primitive.BOX;
    side_wall_primitive.dimensions.resize(3);
    side_wall_primitive.dimensions[0] = side_wall_length; // từ tường sau đến mép trước bàn
    side_wall_primitive.dimensions[1] = wall_thickness;   // độ dày theo trục Y
    side_wall_primitive.dimensions[2] = wall_height;      // chiều cao theo trục Z

    // Thêm tường phía sau robot
    moveit_msgs::msg::CollisionObject back_wall;
    back_wall.header.frame_id = move_group_.getPlanningFrame();
    back_wall.id = "back_wall";

    geometry_msgs::msg::Pose back_wall_pose;
    back_wall_pose.orientation.w = 1.0;
    back_wall_pose.position.x = back_wall_x;
    back_wall_pose.position.y = 0.0;
    back_wall_pose.position.z = wall_height / 2.0;

    back_wall.primitives.push_back(back_wall_primitive);
    back_wall.primitive_poses.push_back(back_wall_pose);
    back_wall.operation = back_wall.ADD;

    // Thêm tường bên trái
    moveit_msgs::msg::CollisionObject left_wall;
    left_wall.header.frame_id = move_group_.getPlanningFrame();
    left_wall.id = "left_wall";

    geometry_msgs::msg::Pose left_wall_pose;
    left_wall_pose.orientation.w = 1.0;
    left_wall_pose.position.x = side_wall_x;
    left_wall_pose.position.y = side_wall_y;
    left_wall_pose.position.z = wall_height / 2.0;

    left_wall.primitives.push_back(side_wall_primitive);
    left_wall.primitive_poses.push_back(left_wall_pose);
    left_wall.operation = left_wall.ADD;

    // Thêm tường bên phải
    moveit_msgs::msg::CollisionObject right_wall;
    right_wall.header.frame_id = move_group_.getPlanningFrame();
    right_wall.id = "right_wall";

    geometry_msgs::msg::Pose right_wall_pose;
    right_wall_pose.orientation.w = 1.0;
    right_wall_pose.position.x = side_wall_x;
    right_wall_pose.position.y = -side_wall_y;
    right_wall_pose.position.z = wall_height / 2.0;

    right_wall.primitives.push_back(side_wall_primitive);
    right_wall.primitive_poses.push_back(right_wall_pose);
    right_wall.operation = right_wall.ADD;

    std::vector<moveit_msgs::msg::CollisionObject> collision_objects;
    collision_objects.push_back(collision_object);
    collision_objects.push_back(back_wall);
    collision_objects.push_back(left_wall);
    collision_objects.push_back(right_wall);

    RCLCPP_INFO(node_->get_logger(), "Adding table and walls into the world...");
    planning_scene_interface_.applyCollisionObjects(collision_objects);
  }

} // namespace ur3_moveit_control

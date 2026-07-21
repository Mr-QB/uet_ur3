#include "ur3_moveit_control/ur3_motion_interface.hpp"
#include <chrono>
#include <cmath>
#include <moveit_msgs/msg/collision_object.hpp>
#include <moveit/robot_trajectory/robot_trajectory.h>
#include <moveit/trajectory_processing/iterative_time_parameterization.h>
#include <shape_msgs/msg/solid_primitive.hpp>

namespace ur3_moveit_control
{

  UR3MotionInterface::UR3MotionInterface(
      const rclcpp::Node::SharedPtr &node,
      const std::string &planning_group)
      : node_(node),
        move_group_(node_, planning_group)
  {
    gazebo_attach_pub_ = node_->create_publisher<std_msgs::msg::Empty>(
      "/pick_box/attach", rclcpp::QoS(1).reliable());
    gazebo_detach_pub_ = node_->create_publisher<std_msgs::msg::Empty>(
      "/pick_box/detach", rclcpp::QoS(1).reliable());

    move_group_.setPlanningTime(5.0);
    move_group_.setNumPlanningAttempts(10);

    // Use the gripper grasp center as the Cartesian pose target whenever the
    // gripper model is present. Keep tool0 as the fallback for the arm-only
    // robot description.
    if (move_group_.getRobotModel()->getLinkModel("gripper_tcp") != nullptr)
    {
      const bool tcp_set = move_group_.setEndEffectorLink("gripper_tcp");
      if (!tcp_set)
      {
        RCLCPP_WARN(
          node_->get_logger(),
          "gripper_tcp exists but could not be selected; using %s instead.",
          move_group_.getEndEffectorLink().c_str());
      }
    }

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

  bool UR3MotionInterface::attachPickBox()
  {
    const std::string object_id = "pick_box";
    const std::string attach_link = "gripper_tcp";
    const std::vector<std::string> touch_links = {
      "gripper_tcp",
      "gripper_body",
      "finger_left",
      "finger_right"
    };

    // First create the physical fixed joint in Gazebo.  Give the ROS-Gazebo
    // bridge and the DetachableJoint system a few simulation iterations to
    // consume the request before the arm starts moving.
    gazebo_attach_pub_->publish(std_msgs::msg::Empty{});
    rclcpp::sleep_for(std::chrono::milliseconds(300));

    // Then mirror that state in MoveIt so collision checking treats the box
    // as carried payload and permits contact with the gripper touch links.
    if (!move_group_.attachObject(object_id, attach_link, touch_links))
    {
      RCLCPP_ERROR(
        node_->get_logger(),
        "Failed to attach %s to %s.",
        object_id.c_str(), attach_link.c_str());
      return false;
    }

    RCLCPP_INFO(
      node_->get_logger(),
      "Attached %s to %s.",
      object_id.c_str(), attach_link.c_str());

    // Give the planning scene monitor time to receive the attached-object update.
    rclcpp::sleep_for(std::chrono::milliseconds(200));
    return true;
  }

  bool UR3MotionInterface::detachPickBox()
  {
    const std::string object_id = "pick_box";

    // Remove the object from MoveIt's AttachedCollisionObject list first so
    // subsequent plans treat it as a world obstacle again.
    if (!move_group_.detachObject(object_id))
    {
      RCLCPP_ERROR(node_->get_logger(), "Failed to detach %s in MoveIt.", object_id.c_str());
      return false;
    }

    gazebo_detach_pub_->publish(std_msgs::msg::Empty{});
    rclcpp::sleep_for(std::chrono::milliseconds(300));
    RCLCPP_INFO(node_->get_logger(), "Detached %s in MoveIt and Gazebo.", object_id.c_str());
    return true;
  }

  bool UR3MotionInterface::moveCartesianZ(double z_offset)
  {
    if (std::abs(z_offset) < 1e-6)
    {
      RCLCPP_ERROR(node_->get_logger(), "Cartesian Z offset must be non-zero.");
      return false;
    }

    const std::string end_effector_link = "gripper_tcp";
    const auto current_pose = move_group_.getCurrentPose(end_effector_link);
    geometry_msgs::msg::Pose target_pose = current_pose.pose;
    target_pose.position.z += z_offset;

    std::vector<geometry_msgs::msg::Pose> waypoints{target_pose};
    moveit_msgs::msg::RobotTrajectory trajectory_msg;

    constexpr double eef_step = 0.005;
    constexpr double jump_threshold = 0.0;
    constexpr double required_fraction = 0.99;

    const double fraction = move_group_.computeCartesianPath(
      waypoints,
      eef_step,
      jump_threshold,
      trajectory_msg,
      true);

    if (fraction < required_fraction)
    {
      RCLCPP_ERROR(
        node_->get_logger(),
        "Cartesian lift planning incomplete: %.1f%% (required %.1f%%).",
        fraction * 100.0, required_fraction * 100.0);
      return false;
    }

    const auto current_state = move_group_.getCurrentState(2.0);
    if (!current_state)
    {
      RCLCPP_ERROR(node_->get_logger(), "Could not read current robot state for trajectory timing.");
      return false;
    }

    robot_trajectory::RobotTrajectory timed_trajectory(
      move_group_.getRobotModel(), move_group_.getName());
    timed_trajectory.setRobotTrajectoryMsg(*current_state, trajectory_msg);

    trajectory_processing::IterativeParabolicTimeParameterization time_parameterization;
    constexpr double lift_velocity_scaling = 0.05;
    constexpr double lift_acceleration_scaling = 0.05;

    if (!time_parameterization.computeTimeStamps(
          timed_trajectory, lift_velocity_scaling, lift_acceleration_scaling))
    {
      RCLCPP_ERROR(node_->get_logger(), "Failed to time-parameterize Cartesian lift.");
      return false;
    }

    timed_trajectory.getRobotTrajectoryMsg(trajectory_msg);

    RCLCPP_INFO(
      node_->get_logger(),
      "Executing Cartesian Z move: %.3f m -> %.3f m (offset %+.3f m).",
      current_pose.pose.position.z, target_pose.position.z, z_offset);

    const auto execution_result = move_group_.execute(trajectory_msg);
    if (execution_result != moveit::core::MoveItErrorCode::SUCCESS)
    {
      RCLCPP_ERROR(node_->get_logger(), "Cartesian lift execution failed.");
      return false;
    }

    RCLCPP_INFO(node_->get_logger(), "Cartesian lift succeeded.");
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
    primitive.dimensions[0] = table_length;    // dimension along X axis
    primitive.dimensions[1] = table_width;     // dimension along Y axis
    primitive.dimensions[2] = table_thickness; // thickness along Z axis

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
    back_wall_primitive.dimensions[0] = wall_thickness;      // thickness along X axis
    back_wall_primitive.dimensions[1] = side_wall_y * 2.0;   // 0.5m offset on both sides
    back_wall_primitive.dimensions[2] = wall_height;         // height along Z axis

    shape_msgs::msg::SolidPrimitive side_wall_primitive;
    side_wall_primitive.type = side_wall_primitive.BOX;
    side_wall_primitive.dimensions.resize(3);
    side_wall_primitive.dimensions[0] = side_wall_length; // from back wall to table's front edge
    side_wall_primitive.dimensions[1] = wall_thickness;   // thickness along Y axis
    side_wall_primitive.dimensions[2] = wall_height;      // height along Z axis

    // Add back wall behind the robot
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

  void UR3MotionInterface::addPickBoxObstacle()
  {
    moveit_msgs::msg::CollisionObject pick_box;
    // Coordinates below are relative to the robot mounting frame.
    pick_box.header.frame_id = "base_link";
    pick_box.id = "pick_box";

    shape_msgs::msg::SolidPrimitive box_primitive;
    box_primitive.type = shape_msgs::msg::SolidPrimitive::BOX;
    box_primitive.dimensions = {0.1, 0.1, 0.1};

    // Gazebo spawns the box at world (0.35, 0.0, 0.825). The robot base is
    // at world z=0.775, so the box center is at z=0.05 in base_link.
    geometry_msgs::msg::Pose box_pose;
    box_pose.orientation.w = 1.0;
    box_pose.position.x = 0.35;
    box_pose.position.y = 0.0;
    box_pose.position.z = 0.05;

    pick_box.primitives.push_back(box_primitive);
    pick_box.primitive_poses.push_back(box_pose);
    pick_box.operation = moveit_msgs::msg::CollisionObject::ADD;

    RCLCPP_INFO(
      node_->get_logger(),
      "Adding pick_box collision object at (0.350, 0.000, 0.050)...");
    planning_scene_interface_.applyCollisionObject(pick_box);
  }

  void UR3MotionInterface::calibObjectHeightEyeInHand(double depth_m, double camera_z_mount_offset)
  {
    geometry_msgs::msg::PoseStamped current_pose = move_group_.getCurrentPose();
    double ee_z = current_pose.pose.position.z;
    double object_height = ee_z - camera_z_mount_offset - depth_m;
    RCLCPP_INFO_THROTTLE(node_->get_logger(), *node_->get_clock(), 2000, "Current EE Z: %.3f, Depth: %.3f, Object Height: %.3f", ee_z, depth_m, object_height);
  }

} // namespace ur3_moveit_control

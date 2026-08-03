#ifndef UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_
#define UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_

#include <memory>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>
#include <geometry_msgs/msg/pose.hpp>
#include <moveit/move_group_interface/move_group_interface.h>
#include <moveit/planning_scene_interface/planning_scene_interface.h>
#include <std_msgs/msg/empty.hpp>

namespace ur3_moveit_control
{

class UR3MotionInterface
{
public:
  explicit UR3MotionInterface(
    const rclcpp::Node::SharedPtr & node,
    const std::string & planning_group = "ur_manipulator");

  bool moveToJointGoal(const std::vector<double> & joint_goal);

  bool moveToPoseGoal(
    const geometry_msgs::msg::Pose & target_pose,
    const std::string & end_effector_link = "");

  bool moveHome();

  bool attachPickBox(
    double object_x = 0.35,
    double object_y = 0.0,
    double object_z = 0.05);

  bool detachPickBox();

  bool prepareNextTrial();

  bool moveCartesian(
    double x_offset,
    double y_offset,
    double z_offset,
    bool allow_pose_fallback = true);

  bool moveToXY(double target_x, double target_y);

  void stop();

  void addTableObstacle();

  void addPickBoxObstacle(
    double x = 0.35, double y = 0.0, double z = 0.05);

  void calibObjectHeightEyeInHand(double depth_m, double camera_z_mount_offset);

private:
  rclcpp::Node::SharedPtr node_;
  moveit::planning_interface::MoveGroupInterface move_group_;
  moveit::planning_interface::PlanningSceneInterface planning_scene_interface_;
  rclcpp::Publisher<std_msgs::msg::Empty>::SharedPtr gazebo_attach_pub_;
  rclcpp::Publisher<std_msgs::msg::Empty>::SharedPtr gazebo_detach_pub_;
};

}  // namespace ur3_moveit_control

#endif  // UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_

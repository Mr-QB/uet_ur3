#ifndef UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_
#define UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_

#include <memory>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>
#include <geometry_msgs/msg/pose.hpp>
#include <moveit/move_group_interface/move_group_interface.h>

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

private:
  rclcpp::Node::SharedPtr node_;
  moveit::planning_interface::MoveGroupInterface move_group_;
};

}  // namespace ur3_moveit_control

#endif  // UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_
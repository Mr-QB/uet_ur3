#ifndef UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_
#define UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_

#include <condition_variable>
#include <memory>
#include <mutex>
#include <string>
#include <vector>

#include <rclcpp/rclcpp.hpp>
#include <geometry_msgs/msg/pose.hpp>
#include <moveit/move_group_interface/move_group_interface.h>
#include <moveit/planning_scene_interface/planning_scene_interface.h>
#include <sensor_msgs/msg/joint_state.hpp>
#include <std_msgs/msg/empty.hpp>
#include <std_msgs/msg/string.hpp>

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

  const std::string & lastPoseErrorMessage() const;

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
  void attachmentStateCallback(const std_msgs::msg::String::SharedPtr msg);

  void jointStateCallback(const sensor_msgs::msg::JointState::SharedPtr msg);

  bool waitForAttachmentState(bool expected_state, double timeout_seconds);

  rclcpp::Node::SharedPtr node_;
  moveit::planning_interface::MoveGroupInterface move_group_;
  moveit::planning_interface::PlanningSceneInterface planning_scene_interface_;
  rclcpp::Publisher<std_msgs::msg::Empty>::SharedPtr gazebo_attach_pub_;
  rclcpp::Publisher<std_msgs::msg::Empty>::SharedPtr gazebo_detach_pub_;
  rclcpp::Subscription<std_msgs::msg::String>::SharedPtr attachment_state_sub_;
  rclcpp::Subscription<sensor_msgs::msg::JointState>::SharedPtr joint_state_sub_;

  std::mutex attachment_state_mutex_;
  std::condition_variable attachment_state_cv_;
  bool attachment_state_{false};
  bool attachment_state_received_{false};

  std::mutex gripper_state_mutex_;
  bool gripper_state_received_{false};
  double gripper_position_{0.0};
  double gripper_effort_{0.0};
  double gripper_close_peak_effort_{0.0};

  std::vector<std::string> successful_waypoint_files_;
  std::vector<double> ik_joint_weights_;
  int ik_random_seed_attempts_{12};
  int ik_max_planning_candidates_{8};
  int ik_max_successful_plans_{3};
  double waypoint_seed_max_position_distance_{0.20};
  double waypoint_seed_max_orientation_distance_{0.35};
  double cartesian_jump_threshold_{1.5};
  double cartesian_max_joint_step_{0.20};
  double attach_confirmation_timeout_{3.0};
  int attach_request_attempts_{3};
  double maximum_attach_tcp_distance_{0.06};
  double maximum_attach_tcp_z_error_{0.04};
  double maximum_gripper_position_for_attach_{0.051};
  double minimum_gripper_effort_for_attach_{0.0001};
  std::string last_pose_error_message_;
};

}  // namespace ur3_moveit_control

#endif  // UR3_MOVEIT_CONTROL__UR3_MOTION_INTERFACE_HPP_

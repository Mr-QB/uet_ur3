#include <memory>
#include <thread>
#include <mutex>

#include <rclcpp/rclcpp.hpp>
#include <rclcpp_action/rclcpp_action.hpp>
#include <geometry_msgs/msg/pose_stamped.hpp>
#include <sensor_msgs/msg/joint_state.hpp>
#include <sensor_msgs/msg/image.hpp>
#include <visualization_msgs/msg/marker_array.hpp>

#include "ur3_moveit_control/action/ur3_control.hpp"
#include "ur3_moveit_control/ur3_motion_interface.hpp"

using UR3Control = ur3_moveit_control::action::UR3Control;
using GoalHandleUR3Control = rclcpp_action::ServerGoalHandle<UR3Control>;

class UR3ControlNode : public rclcpp::Node
{
public:
  UR3ControlNode()
  : Node("ur3_control_node", rclcpp::NodeOptions().automatically_declare_parameters_from_overrides(true))
  {
  }

  void init()
  {
    ur3_motion_ = std::make_shared<ur3_moveit_control::UR3MotionInterface>(shared_from_this(), "ur_manipulator");

    action_server_ = rclcpp_action::create_server<UR3Control>(
      this,
      "ur3_control",
      std::bind(&UR3ControlNode::handle_goal, this, std::placeholders::_1, std::placeholders::_2),
      std::bind(&UR3ControlNode::handle_cancel, this, std::placeholders::_1),
      std::bind(&UR3ControlNode::handle_accepted, this, std::placeholders::_1));

    ur3_motion_->addTableObstacle();

    auto marker_qos = rclcpp::QoS(1).reliable().transient_local();
    fixed_bottle_marker_pub_ =
      this->create_publisher<visualization_msgs::msg::MarkerArray>(
        "/fixed_bottle_marker", marker_qos);
    publish_fixed_bottle_marker();

    depth_sub_ = this->create_subscription<sensor_msgs::msg::Image>(
      "/camera/camera/depth/image_rect_raw",
      rclcpp::SensorDataQoS(),
      std::bind(&UR3ControlNode::depth_callback, this, std::placeholders::_1));

    RCLCPP_INFO(this->get_logger(), "UR3 Action Server initialized with Camera Calibration.");
  }

private:
  std::shared_ptr<ur3_moveit_control::UR3MotionInterface> ur3_motion_;
  rclcpp_action::Server<UR3Control>::SharedPtr action_server_;
  rclcpp::Subscription<sensor_msgs::msg::Image>::SharedPtr depth_sub_;
  rclcpp::Publisher<visualization_msgs::msg::MarkerArray>::SharedPtr
    fixed_bottle_marker_pub_;
  double camera_z_mount_offset_ = 0.05; // Offset Z từ tay gắp tới camera (m)
  
  std::shared_ptr<GoalHandleUR3Control> active_goal_;
  std::mutex mutex_;
  std::thread execution_thread_;

  void publish_fixed_bottle_marker()
  {
    constexpr double bottle_x = 0.62;
    constexpr double bottle_y = 0.0;
    constexpr double bottle_z = 0.05;

    visualization_msgs::msg::MarkerArray markers;
    const auto add_marker = [this, &markers](
      int id,
      int type,
      double center_x,
      double center_y,
      double center_z,
      double scale_x,
      double scale_y,
      double scale_z,
      float red,
      float green,
      float blue)
    {
      visualization_msgs::msg::Marker marker;
      marker.header.frame_id = "base_link";
      marker.header.stamp = this->now();
      marker.ns = "fixed_bottle";
      marker.id = id;
      marker.type = type;
      marker.action = visualization_msgs::msg::Marker::ADD;
      marker.pose.position.x = center_x;
      marker.pose.position.y = center_y;
      marker.pose.position.z = center_z;
      marker.pose.orientation.w = 1.0;
      marker.scale.x = scale_x;
      marker.scale.y = scale_y;
      marker.scale.z = scale_z;
      marker.color.r = red;
      marker.color.g = green;
      marker.color.b = blue;
      marker.color.a = 0.90F;
      marker.frame_locked = true;
      markers.markers.push_back(marker);
    };

    // These dimensions and offsets match models/bottle.sdf and the MoveIt
    // collision geometry used when the bottle becomes attached.
    add_marker(
      0, visualization_msgs::msg::Marker::CYLINDER,
      bottle_x, bottle_y, bottle_z + 0.030,
      0.100, 0.100, 0.160,
      1.00F, 0.52F, 0.06F);
    add_marker(
      1, visualization_msgs::msg::Marker::SPHERE,
      bottle_x, bottle_y, bottle_z + 0.110,
      0.100, 0.100, 0.100,
      1.00F, 0.52F, 0.06F);
    add_marker(
      2, visualization_msgs::msg::Marker::CYLINDER,
      bottle_x, bottle_y, bottle_z + 0.175,
      0.044, 0.044, 0.040,
      1.00F, 0.52F, 0.06F);
    add_marker(
      3, visualization_msgs::msg::Marker::CYLINDER,
      bottle_x, bottle_y, bottle_z + 0.2025,
      0.060, 0.060, 0.015,
      0.72F, 0.72F, 0.72F);

    fixed_bottle_marker_pub_->publish(markers);
    RCLCPP_INFO(
      this->get_logger(),
      "Published fixed bottle RViz marker at base_link (%.3f, %.3f, %.3f).",
      bottle_x, bottle_y, bottle_z);
  }

  void depth_callback(const sensor_msgs::msg::Image::SharedPtr msg)
  {
    if (msg->encoding != "16UC1") {
      RCLCPP_WARN_ONCE(this->get_logger(), "Camera encoding không phải 16UC1. Không thể đọc!");
      return;
    }

    int center_x = msg->width / 2;
    int center_y = msg->height / 2;
    int index = (center_y * msg->step) + (center_x * 2);

    if (static_cast<size_t>(index + 1) >= msg->data.size()) {
      return;
    }

    uint16_t depth_mm = msg->data[index] | (msg->data[index + 1] << 8);
    double depth_m = static_cast<double>(depth_mm) / 1000.0;

    if (depth_m > 0.0) {
      try {
        ur3_motion_->calibObjectHeightEyeInHand(depth_m, camera_z_mount_offset_);
      } catch (const std::exception &e) {
        RCLCPP_WARN_THROTTLE(this->get_logger(), *this->get_clock(), 2000, "Đang đợi TF tay máy để tính toán chiều cao...");
      }
    }
  }

  rclcpp_action::GoalResponse handle_goal(
    const rclcpp_action::GoalUUID & uuid,
    std::shared_ptr<const UR3Control::Goal> goal)
  {
    RCLCPP_INFO(this->get_logger(), "Received goal request. Canceling active goal if any.");
    (void)uuid;
    (void)goal;
    
    std::lock_guard<std::mutex> lock(mutex_);
    if (active_goal_ && active_goal_->is_active()) {
        RCLCPP_WARN(this->get_logger(), "Stopping current execution for new goal.");
        ur3_motion_->stop();
    }

    return rclcpp_action::GoalResponse::ACCEPT_AND_EXECUTE;
  }

  rclcpp_action::CancelResponse handle_cancel(
    const std::shared_ptr<GoalHandleUR3Control> goal_handle)
  {
    RCLCPP_INFO(this->get_logger(), "Received request to cancel goal");
    (void)goal_handle;

    std::lock_guard<std::mutex> lock(mutex_);
    ur3_motion_->stop();
    return rclcpp_action::CancelResponse::ACCEPT;
  }

  void handle_accepted(const std::shared_ptr<GoalHandleUR3Control> goal_handle)
  {
    std::lock_guard<std::mutex> lock(mutex_);
    if (execution_thread_.joinable()) {
      execution_thread_.join();
    }
    active_goal_ = goal_handle;
    execution_thread_ = std::thread(std::bind(&UR3ControlNode::execute, this, goal_handle));
  }

  void execute(const std::shared_ptr<GoalHandleUR3Control> goal_handle)
  {
    const auto goal = goal_handle->get_goal();
    auto feedback = std::make_shared<UR3Control::Feedback>();
    auto result = std::make_shared<UR3Control::Result>();

    feedback->state = "Planning and Executing";
    goal_handle->publish_feedback(feedback);

    bool success = false;
    if (goal->command_type == UR3Control::Goal::MOVE_HOME) {
      success = ur3_motion_->moveHome();
    } else if (goal->command_type == UR3Control::Goal::MOVE_JOINT) {
      success = ur3_motion_->moveToJointGoal(goal->joint_goal.position);
    } else if (goal->command_type == UR3Control::Goal::MOVE_POSE) {
      success = ur3_motion_->moveToPoseGoal(goal->pose_goal.pose);
    } else if (goal->command_type == UR3Control::Goal::ATTACH_AND_LIFT) {
      feedback->state = "Attaching pick_box to gripper_tcp";
      goal_handle->publish_feedback(feedback);

      const bool attached = ur3_motion_->attachPickBox(
        goal->object_x,
        goal->object_y,
        goal->object_z);
      if (attached) {
        feedback->state = "Executing Cartesian lift";
        goal_handle->publish_feedback(feedback);
        success = ur3_motion_->moveCartesian(0.0, 0.0, goal->cartesian_z_offset);
      }
    } else if (goal->command_type == UR3Control::Goal::DETACH_OBJECT) {
      feedback->state = "Detaching pick_box from gripper_tcp";
      goal_handle->publish_feedback(feedback);
      success = ur3_motion_->detachPickBox();
    } else if (goal->command_type == UR3Control::Goal::PREPARE_NEXT_TRIAL) {
      feedback->state = "Preparing reusable pick_box for the next trial";
      goal_handle->publish_feedback(feedback);
      success = ur3_motion_->prepareNextTrial();
    } else if (goal->command_type == UR3Control::Goal::MOVE_CARTESIAN) {
      feedback->state = "Executing Cartesian XYZ motion from current pose";
      goal_handle->publish_feedback(feedback);
      success = ur3_motion_->moveCartesian(
        goal->cartesian_x_offset,
        goal->cartesian_y_offset,
        goal->cartesian_z_offset);
    } else if (goal->command_type == UR3Control::Goal::MOVE_CARTESIAN_STRICT) {
      feedback->state = "Executing strict Cartesian grasp approach";
      goal_handle->publish_feedback(feedback);
      success = ur3_motion_->moveCartesian(
        goal->cartesian_x_offset,
        goal->cartesian_y_offset,
        goal->cartesian_z_offset,
        false);
    } else if (goal->command_type == UR3Control::Goal::MOVE_TO_XY) {
      feedback->state = "Moving to absolute XY while keeping current Z";
      goal_handle->publish_feedback(feedback);
      success = ur3_motion_->moveToXY(goal->target_x, goal->target_y);
    } else {
      RCLCPP_ERROR(this->get_logger(), "Invalid command type!");
    }

    if (goal_handle->is_canceling()) {
      result->success = false;
      result->message = "Goal canceled by user or new goal";
      goal_handle->canceled(result);
      RCLCPP_INFO(this->get_logger(), "Goal canceled");
      return;
    }

    if (success) {
      result->success = true;
      result->message = "Goal succeeded";
      goal_handle->succeed(result);
      RCLCPP_INFO(this->get_logger(), "Goal succeeded");
    } else {
      result->success = false;
      if (goal->command_type == UR3Control::Goal::MOVE_POSE &&
          !ur3_motion_->lastPoseErrorMessage().empty()) {
        result->message = ur3_motion_->lastPoseErrorMessage();
      } else {
        result->message = "Goal failed or was preempted";
      }
      goal_handle->abort(result);
      RCLCPP_INFO(this->get_logger(), "Goal failed");
    }
  }
};

int main(int argc, char ** argv)
{
  rclcpp::init(argc, argv);
  
  auto node = std::make_shared<UR3ControlNode>();
  
  rclcpp::executors::MultiThreadedExecutor executor;
  executor.add_node(node);

  std::thread spinner([&executor]() {
    executor.spin();
  });

  node->init();

  if (spinner.joinable()) {
    spinner.join();
  }

  rclcpp::shutdown();
  return 0;
}

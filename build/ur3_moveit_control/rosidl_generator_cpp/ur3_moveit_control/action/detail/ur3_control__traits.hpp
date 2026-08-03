// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur3_moveit_control:action/UR3Control.idl
// generated code does not contain a copyright notice

#ifndef UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__TRAITS_HPP_
#define UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur3_moveit_control/action/detail/ur3_control__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'joint_goal'
#include "sensor_msgs/msg/detail/joint_state__traits.hpp"
// Member 'pose_goal'
#include "geometry_msgs/msg/detail/pose_stamped__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: command_type
  {
    out << "command_type: ";
    rosidl_generator_traits::value_to_yaml(msg.command_type, out);
    out << ", ";
  }

  // member: joint_goal
  {
    out << "joint_goal: ";
    to_flow_style_yaml(msg.joint_goal, out);
    out << ", ";
  }

  // member: pose_goal
  {
    out << "pose_goal: ";
    to_flow_style_yaml(msg.pose_goal, out);
    out << ", ";
  }

  // member: cartesian_x_offset
  {
    out << "cartesian_x_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_x_offset, out);
    out << ", ";
  }

  // member: cartesian_y_offset
  {
    out << "cartesian_y_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_y_offset, out);
    out << ", ";
  }

  // member: cartesian_z_offset
  {
    out << "cartesian_z_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_z_offset, out);
    out << ", ";
  }

  // member: target_x
  {
    out << "target_x: ";
    rosidl_generator_traits::value_to_yaml(msg.target_x, out);
    out << ", ";
  }

  // member: target_y
  {
    out << "target_y: ";
    rosidl_generator_traits::value_to_yaml(msg.target_y, out);
    out << ", ";
  }

  // member: object_x
  {
    out << "object_x: ";
    rosidl_generator_traits::value_to_yaml(msg.object_x, out);
    out << ", ";
  }

  // member: object_y
  {
    out << "object_y: ";
    rosidl_generator_traits::value_to_yaml(msg.object_y, out);
    out << ", ";
  }

  // member: object_z
  {
    out << "object_z: ";
    rosidl_generator_traits::value_to_yaml(msg.object_z, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: command_type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "command_type: ";
    rosidl_generator_traits::value_to_yaml(msg.command_type, out);
    out << "\n";
  }

  // member: joint_goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "joint_goal:\n";
    to_block_style_yaml(msg.joint_goal, out, indentation + 2);
  }

  // member: pose_goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose_goal:\n";
    to_block_style_yaml(msg.pose_goal, out, indentation + 2);
  }

  // member: cartesian_x_offset
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "cartesian_x_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_x_offset, out);
    out << "\n";
  }

  // member: cartesian_y_offset
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "cartesian_y_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_y_offset, out);
    out << "\n";
  }

  // member: cartesian_z_offset
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "cartesian_z_offset: ";
    rosidl_generator_traits::value_to_yaml(msg.cartesian_z_offset, out);
    out << "\n";
  }

  // member: target_x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "target_x: ";
    rosidl_generator_traits::value_to_yaml(msg.target_x, out);
    out << "\n";
  }

  // member: target_y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "target_y: ";
    rosidl_generator_traits::value_to_yaml(msg.target_y, out);
    out << "\n";
  }

  // member: object_x
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "object_x: ";
    rosidl_generator_traits::value_to_yaml(msg.object_x, out);
    out << "\n";
  }

  // member: object_y
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "object_y: ";
    rosidl_generator_traits::value_to_yaml(msg.object_y, out);
    out << "\n";
  }

  // member: object_z
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "object_z: ";
    rosidl_generator_traits::value_to_yaml(msg.object_z, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_Goal & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_Goal>()
{
  return "ur3_moveit_control::action::UR3Control_Goal";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_Goal>()
{
  return "ur3_moveit_control/action/UR3Control_Goal";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_Goal>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::PoseStamped>::value && has_fixed_size<sensor_msgs::msg::JointState>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_Goal>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::PoseStamped>::value && has_bounded_size<sensor_msgs::msg::JointState>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }

  // member: message
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_Result & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_Result>()
{
  return "ur3_moveit_control::action::UR3Control_Result";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_Result>()
{
  return "ur3_moveit_control/action/UR3Control_Result";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_Result>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_Result>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: state
  {
    out << "state: ";
    rosidl_generator_traits::value_to_yaml(msg.state, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "state: ";
    rosidl_generator_traits::value_to_yaml(msg.state, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_Feedback & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_Feedback>()
{
  return "ur3_moveit_control::action::UR3Control_Feedback";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_Feedback>()
{
  return "ur3_moveit_control/action/UR3Control_Feedback";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_Feedback>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_Feedback>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "ur3_moveit_control/action/detail/ur3_control__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_SendGoal_Request & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_SendGoal_Request>()
{
  return "ur3_moveit_control::action::UR3Control_SendGoal_Request";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_SendGoal_Request>()
{
  return "ur3_moveit_control/action/UR3Control_SendGoal_Request";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value && has_fixed_size<ur3_moveit_control::action::UR3Control_Goal>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value && has_bounded_size<ur3_moveit_control::action::UR3Control_Goal>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_SendGoal_Response & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_SendGoal_Response>()
{
  return "ur3_moveit_control::action::UR3Control_SendGoal_Response";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_SendGoal_Response>()
{
  return "ur3_moveit_control/action/UR3Control_SendGoal_Response";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_SendGoal>()
{
  return "ur3_moveit_control::action::UR3Control_SendGoal";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_SendGoal>()
{
  return "ur3_moveit_control/action/UR3Control_SendGoal";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<ur3_moveit_control::action::UR3Control_SendGoal_Request>::value &&
    has_fixed_size<ur3_moveit_control::action::UR3Control_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<ur3_moveit_control::action::UR3Control_SendGoal_Request>::value &&
    has_bounded_size<ur3_moveit_control::action::UR3Control_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<ur3_moveit_control::action::UR3Control_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<ur3_moveit_control::action::UR3Control_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ur3_moveit_control::action::UR3Control_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_GetResult_Request & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_GetResult_Request>()
{
  return "ur3_moveit_control::action::UR3Control_GetResult_Request";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_GetResult_Request>()
{
  return "ur3_moveit_control/action/UR3Control_GetResult_Request";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_GetResult_Response & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_GetResult_Response>()
{
  return "ur3_moveit_control::action::UR3Control_GetResult_Response";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_GetResult_Response>()
{
  return "ur3_moveit_control/action/UR3Control_GetResult_Response";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<ur3_moveit_control::action::UR3Control_Result>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<ur3_moveit_control::action::UR3Control_Result>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_GetResult>()
{
  return "ur3_moveit_control::action::UR3Control_GetResult";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_GetResult>()
{
  return "ur3_moveit_control/action/UR3Control_GetResult";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<ur3_moveit_control::action::UR3Control_GetResult_Request>::value &&
    has_fixed_size<ur3_moveit_control::action::UR3Control_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<ur3_moveit_control::action::UR3Control_GetResult_Request>::value &&
    has_bounded_size<ur3_moveit_control::action::UR3Control_GetResult_Response>::value
  >
{
};

template<>
struct is_service<ur3_moveit_control::action::UR3Control_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<ur3_moveit_control::action::UR3Control_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ur3_moveit_control::action::UR3Control_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "ur3_moveit_control/action/detail/ur3_control__traits.hpp"

namespace ur3_moveit_control
{

namespace action
{

inline void to_flow_style_yaml(
  const UR3Control_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UR3Control_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UR3Control_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ur3_moveit_control

namespace rosidl_generator_traits
{

[[deprecated("use ur3_moveit_control::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur3_moveit_control::action::UR3Control_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur3_moveit_control::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur3_moveit_control::action::to_yaml() instead")]]
inline std::string to_yaml(const ur3_moveit_control::action::UR3Control_FeedbackMessage & msg)
{
  return ur3_moveit_control::action::to_yaml(msg);
}

template<>
inline const char * data_type<ur3_moveit_control::action::UR3Control_FeedbackMessage>()
{
  return "ur3_moveit_control::action::UR3Control_FeedbackMessage";
}

template<>
inline const char * name<ur3_moveit_control::action::UR3Control_FeedbackMessage>()
{
  return "ur3_moveit_control/action/UR3Control_FeedbackMessage";
}

template<>
struct has_fixed_size<ur3_moveit_control::action::UR3Control_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value && has_fixed_size<ur3_moveit_control::action::UR3Control_Feedback>::value> {};

template<>
struct has_bounded_size<ur3_moveit_control::action::UR3Control_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value && has_bounded_size<ur3_moveit_control::action::UR3Control_Feedback>::value> {};

template<>
struct is_message<ur3_moveit_control::action::UR3Control_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<ur3_moveit_control::action::UR3Control>
  : std::true_type
{
};

template<>
struct is_action_goal<ur3_moveit_control::action::UR3Control_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<ur3_moveit_control::action::UR3Control_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<ur3_moveit_control::action::UR3Control_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__TRAITS_HPP_

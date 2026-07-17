// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur_dashboard_msgs:srv/GetRobotModel.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__TRAITS_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur_dashboard_msgs/srv/detail/get_robot_model__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ur_dashboard_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetRobotModel_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetRobotModel_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetRobotModel_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace rosidl_generator_traits
{

[[deprecated("use ur_dashboard_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur_dashboard_msgs::srv::GetRobotModel_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::srv::GetRobotModel_Request & msg)
{
  return ur_dashboard_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GetRobotModel_Request>()
{
  return "ur_dashboard_msgs::srv::GetRobotModel_Request";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GetRobotModel_Request>()
{
  return "ur_dashboard_msgs/srv/GetRobotModel_Request";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GetRobotModel_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GetRobotModel_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ur_dashboard_msgs::srv::GetRobotModel_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ur_dashboard_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetRobotModel_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: answer
  {
    out << "answer: ";
    rosidl_generator_traits::value_to_yaml(msg.answer, out);
    out << ", ";
  }

  // member: robot_model
  {
    out << "robot_model: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_model, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetRobotModel_Response & msg,
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

  // member: answer
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "answer: ";
    rosidl_generator_traits::value_to_yaml(msg.answer, out);
    out << "\n";
  }

  // member: robot_model
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "robot_model: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_model, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetRobotModel_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace ur_dashboard_msgs

namespace rosidl_generator_traits
{

[[deprecated("use ur_dashboard_msgs::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur_dashboard_msgs::srv::GetRobotModel_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::srv::GetRobotModel_Response & msg)
{
  return ur_dashboard_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GetRobotModel_Response>()
{
  return "ur_dashboard_msgs::srv::GetRobotModel_Response";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GetRobotModel_Response>()
{
  return "ur_dashboard_msgs/srv/GetRobotModel_Response";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GetRobotModel_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GetRobotModel_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur_dashboard_msgs::srv::GetRobotModel_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GetRobotModel>()
{
  return "ur_dashboard_msgs::srv::GetRobotModel";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GetRobotModel>()
{
  return "ur_dashboard_msgs/srv/GetRobotModel";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GetRobotModel>
  : std::integral_constant<
    bool,
    has_fixed_size<ur_dashboard_msgs::srv::GetRobotModel_Request>::value &&
    has_fixed_size<ur_dashboard_msgs::srv::GetRobotModel_Response>::value
  >
{
};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GetRobotModel>
  : std::integral_constant<
    bool,
    has_bounded_size<ur_dashboard_msgs::srv::GetRobotModel_Request>::value &&
    has_bounded_size<ur_dashboard_msgs::srv::GetRobotModel_Response>::value
  >
{
};

template<>
struct is_service<ur_dashboard_msgs::srv::GetRobotModel>
  : std::true_type
{
};

template<>
struct is_service_request<ur_dashboard_msgs::srv::GetRobotModel_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ur_dashboard_msgs::srv::GetRobotModel_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__TRAITS_HPP_

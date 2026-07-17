// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur_dashboard_msgs:msg/SafetyStatus.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__TRAITS_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur_dashboard_msgs/msg/detail/safety_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ur_dashboard_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const SafetyStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SafetyStatus & msg,
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
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SafetyStatus & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace ur_dashboard_msgs

namespace rosidl_generator_traits
{

[[deprecated("use ur_dashboard_msgs::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ur_dashboard_msgs::msg::SafetyStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::msg::SafetyStatus & msg)
{
  return ur_dashboard_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::msg::SafetyStatus>()
{
  return "ur_dashboard_msgs::msg::SafetyStatus";
}

template<>
inline const char * name<ur_dashboard_msgs::msg::SafetyStatus>()
{
  return "ur_dashboard_msgs/msg/SafetyStatus";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::msg::SafetyStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::msg::SafetyStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ur_dashboard_msgs::msg::SafetyStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__TRAITS_HPP_

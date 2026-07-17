// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur_dashboard_msgs:msg/VersionInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__TRAITS_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur_dashboard_msgs/msg/detail/version_information__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ur_dashboard_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const VersionInformation & msg,
  std::ostream & out)
{
  out << "{";
  // member: major
  {
    out << "major: ";
    rosidl_generator_traits::value_to_yaml(msg.major, out);
    out << ", ";
  }

  // member: minor
  {
    out << "minor: ";
    rosidl_generator_traits::value_to_yaml(msg.minor, out);
    out << ", ";
  }

  // member: bugfix
  {
    out << "bugfix: ";
    rosidl_generator_traits::value_to_yaml(msg.bugfix, out);
    out << ", ";
  }

  // member: build
  {
    out << "build: ";
    rosidl_generator_traits::value_to_yaml(msg.build, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const VersionInformation & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: major
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "major: ";
    rosidl_generator_traits::value_to_yaml(msg.major, out);
    out << "\n";
  }

  // member: minor
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "minor: ";
    rosidl_generator_traits::value_to_yaml(msg.minor, out);
    out << "\n";
  }

  // member: bugfix
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "bugfix: ";
    rosidl_generator_traits::value_to_yaml(msg.bugfix, out);
    out << "\n";
  }

  // member: build
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "build: ";
    rosidl_generator_traits::value_to_yaml(msg.build, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const VersionInformation & msg, bool use_flow_style = false)
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
  const ur_dashboard_msgs::msg::VersionInformation & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::msg::VersionInformation & msg)
{
  return ur_dashboard_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::msg::VersionInformation>()
{
  return "ur_dashboard_msgs::msg::VersionInformation";
}

template<>
inline const char * name<ur_dashboard_msgs::msg::VersionInformation>()
{
  return "ur_dashboard_msgs/msg/VersionInformation";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::msg::VersionInformation>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::msg::VersionInformation>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ur_dashboard_msgs::msg::VersionInformation>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__TRAITS_HPP_

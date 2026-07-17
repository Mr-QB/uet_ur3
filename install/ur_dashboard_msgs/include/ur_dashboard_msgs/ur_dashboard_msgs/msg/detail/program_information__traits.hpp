// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__TRAITS_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur_dashboard_msgs/msg/detail/program_information__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ur_dashboard_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const ProgramInformation & msg,
  std::ostream & out)
{
  out << "{";
  // member: name
  {
    out << "name: ";
    rosidl_generator_traits::value_to_yaml(msg.name, out);
    out << ", ";
  }

  // member: description
  {
    out << "description: ";
    rosidl_generator_traits::value_to_yaml(msg.description, out);
    out << ", ";
  }

  // member: created_date
  {
    out << "created_date: ";
    rosidl_generator_traits::value_to_yaml(msg.created_date, out);
    out << ", ";
  }

  // member: last_saved_date
  {
    out << "last_saved_date: ";
    rosidl_generator_traits::value_to_yaml(msg.last_saved_date, out);
    out << ", ";
  }

  // member: last_modified_date
  {
    out << "last_modified_date: ";
    rosidl_generator_traits::value_to_yaml(msg.last_modified_date, out);
    out << ", ";
  }

  // member: program_state
  {
    out << "program_state: ";
    rosidl_generator_traits::value_to_yaml(msg.program_state, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ProgramInformation & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: name
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "name: ";
    rosidl_generator_traits::value_to_yaml(msg.name, out);
    out << "\n";
  }

  // member: description
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "description: ";
    rosidl_generator_traits::value_to_yaml(msg.description, out);
    out << "\n";
  }

  // member: created_date
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "created_date: ";
    rosidl_generator_traits::value_to_yaml(msg.created_date, out);
    out << "\n";
  }

  // member: last_saved_date
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "last_saved_date: ";
    rosidl_generator_traits::value_to_yaml(msg.last_saved_date, out);
    out << "\n";
  }

  // member: last_modified_date
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "last_modified_date: ";
    rosidl_generator_traits::value_to_yaml(msg.last_modified_date, out);
    out << "\n";
  }

  // member: program_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "program_state: ";
    rosidl_generator_traits::value_to_yaml(msg.program_state, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ProgramInformation & msg, bool use_flow_style = false)
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
  const ur_dashboard_msgs::msg::ProgramInformation & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::msg::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::msg::ProgramInformation & msg)
{
  return ur_dashboard_msgs::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::msg::ProgramInformation>()
{
  return "ur_dashboard_msgs::msg::ProgramInformation";
}

template<>
inline const char * name<ur_dashboard_msgs::msg::ProgramInformation>()
{
  return "ur_dashboard_msgs/msg/ProgramInformation";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::msg::ProgramInformation>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::msg::ProgramInformation>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur_dashboard_msgs::msg::ProgramInformation>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__TRAITS_HPP_

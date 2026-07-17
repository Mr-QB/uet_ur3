// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ur_dashboard_msgs:srv/GenerateFlightReport.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__TRAITS_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ur_dashboard_msgs/srv/detail/generate_flight_report__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ur_dashboard_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GenerateFlightReport_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: report_type
  {
    out << "report_type: ";
    rosidl_generator_traits::value_to_yaml(msg.report_type, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GenerateFlightReport_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: report_type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "report_type: ";
    rosidl_generator_traits::value_to_yaml(msg.report_type, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GenerateFlightReport_Request & msg, bool use_flow_style = false)
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
  const ur_dashboard_msgs::srv::GenerateFlightReport_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::srv::GenerateFlightReport_Request & msg)
{
  return ur_dashboard_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GenerateFlightReport_Request>()
{
  return "ur_dashboard_msgs::srv::GenerateFlightReport_Request";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GenerateFlightReport_Request>()
{
  return "ur_dashboard_msgs/srv/GenerateFlightReport_Request";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GenerateFlightReport_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GenerateFlightReport_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur_dashboard_msgs::srv::GenerateFlightReport_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ur_dashboard_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const GenerateFlightReport_Response & msg,
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

  // member: report_id
  {
    out << "report_id: ";
    rosidl_generator_traits::value_to_yaml(msg.report_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GenerateFlightReport_Response & msg,
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

  // member: report_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "report_id: ";
    rosidl_generator_traits::value_to_yaml(msg.report_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GenerateFlightReport_Response & msg, bool use_flow_style = false)
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
  const ur_dashboard_msgs::srv::GenerateFlightReport_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ur_dashboard_msgs::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ur_dashboard_msgs::srv::to_yaml() instead")]]
inline std::string to_yaml(const ur_dashboard_msgs::srv::GenerateFlightReport_Response & msg)
{
  return ur_dashboard_msgs::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GenerateFlightReport_Response>()
{
  return "ur_dashboard_msgs::srv::GenerateFlightReport_Response";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GenerateFlightReport_Response>()
{
  return "ur_dashboard_msgs/srv/GenerateFlightReport_Response";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GenerateFlightReport_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GenerateFlightReport_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ur_dashboard_msgs::srv::GenerateFlightReport_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ur_dashboard_msgs::srv::GenerateFlightReport>()
{
  return "ur_dashboard_msgs::srv::GenerateFlightReport";
}

template<>
inline const char * name<ur_dashboard_msgs::srv::GenerateFlightReport>()
{
  return "ur_dashboard_msgs/srv/GenerateFlightReport";
}

template<>
struct has_fixed_size<ur_dashboard_msgs::srv::GenerateFlightReport>
  : std::integral_constant<
    bool,
    has_fixed_size<ur_dashboard_msgs::srv::GenerateFlightReport_Request>::value &&
    has_fixed_size<ur_dashboard_msgs::srv::GenerateFlightReport_Response>::value
  >
{
};

template<>
struct has_bounded_size<ur_dashboard_msgs::srv::GenerateFlightReport>
  : std::integral_constant<
    bool,
    has_bounded_size<ur_dashboard_msgs::srv::GenerateFlightReport_Request>::value &&
    has_bounded_size<ur_dashboard_msgs::srv::GenerateFlightReport_Response>::value
  >
{
};

template<>
struct is_service<ur_dashboard_msgs::srv::GenerateFlightReport>
  : std::true_type
{
};

template<>
struct is_service_request<ur_dashboard_msgs::srv::GenerateFlightReport_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ur_dashboard_msgs::srv::GenerateFlightReport_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__TRAITS_HPP_

// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GenerateFlightReport.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/generate_flight_report__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GenerateFlightReport_Request_report_type
{
public:
  Init_GenerateFlightReport_Request_report_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Request report_type(::ur_dashboard_msgs::srv::GenerateFlightReport_Request::_report_type_type arg)
  {
    msg_.report_type = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GenerateFlightReport_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_GenerateFlightReport_Request_report_type();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GenerateFlightReport_Response_report_id
{
public:
  explicit Init_GenerateFlightReport_Response_report_id(::ur_dashboard_msgs::srv::GenerateFlightReport_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Response report_id(::ur_dashboard_msgs::srv::GenerateFlightReport_Response::_report_id_type arg)
  {
    msg_.report_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Response msg_;
};

class Init_GenerateFlightReport_Response_answer
{
public:
  explicit Init_GenerateFlightReport_Response_answer(::ur_dashboard_msgs::srv::GenerateFlightReport_Response & msg)
  : msg_(msg)
  {}
  Init_GenerateFlightReport_Response_report_id answer(::ur_dashboard_msgs::srv::GenerateFlightReport_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GenerateFlightReport_Response_report_id(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Response msg_;
};

class Init_GenerateFlightReport_Response_success
{
public:
  Init_GenerateFlightReport_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GenerateFlightReport_Response_answer success(::ur_dashboard_msgs::srv::GenerateFlightReport_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GenerateFlightReport_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateFlightReport_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GenerateFlightReport_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GenerateFlightReport_Response_success();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_FLIGHT_REPORT__BUILDER_HPP_

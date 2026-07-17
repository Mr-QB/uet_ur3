// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GetSerialNumber.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/get_serial_number__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GetSerialNumber_Request>()
{
  return ::ur_dashboard_msgs::srv::GetSerialNumber_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GetSerialNumber_Response_success
{
public:
  explicit Init_GetSerialNumber_Response_success(::ur_dashboard_msgs::srv::GetSerialNumber_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GetSerialNumber_Response success(::ur_dashboard_msgs::srv::GetSerialNumber_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSerialNumber_Response msg_;
};

class Init_GetSerialNumber_Response_answer
{
public:
  explicit Init_GetSerialNumber_Response_answer(::ur_dashboard_msgs::srv::GetSerialNumber_Response & msg)
  : msg_(msg)
  {}
  Init_GetSerialNumber_Response_success answer(::ur_dashboard_msgs::srv::GetSerialNumber_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GetSerialNumber_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSerialNumber_Response msg_;
};

class Init_GetSerialNumber_Response_serial_number
{
public:
  Init_GetSerialNumber_Response_serial_number()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetSerialNumber_Response_answer serial_number(::ur_dashboard_msgs::srv::GetSerialNumber_Response::_serial_number_type arg)
  {
    msg_.serial_number = std::move(arg);
    return Init_GetSerialNumber_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSerialNumber_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GetSerialNumber_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GetSerialNumber_Response_serial_number();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SERIAL_NUMBER__BUILDER_HPP_

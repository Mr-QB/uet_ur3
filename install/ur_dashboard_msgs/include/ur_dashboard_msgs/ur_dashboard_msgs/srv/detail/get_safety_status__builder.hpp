// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GetSafetyStatus.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SAFETY_STATUS__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SAFETY_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/get_safety_status__struct.hpp"
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
auto build<::ur_dashboard_msgs::srv::GetSafetyStatus_Request>()
{
  return ::ur_dashboard_msgs::srv::GetSafetyStatus_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GetSafetyStatus_Response_safety_status
{
public:
  explicit Init_GetSafetyStatus_Response_safety_status(::ur_dashboard_msgs::srv::GetSafetyStatus_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GetSafetyStatus_Response safety_status(::ur_dashboard_msgs::srv::GetSafetyStatus_Response::_safety_status_type arg)
  {
    msg_.safety_status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSafetyStatus_Response msg_;
};

class Init_GetSafetyStatus_Response_success
{
public:
  explicit Init_GetSafetyStatus_Response_success(::ur_dashboard_msgs::srv::GetSafetyStatus_Response & msg)
  : msg_(msg)
  {}
  Init_GetSafetyStatus_Response_safety_status success(::ur_dashboard_msgs::srv::GetSafetyStatus_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GetSafetyStatus_Response_safety_status(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSafetyStatus_Response msg_;
};

class Init_GetSafetyStatus_Response_answer
{
public:
  Init_GetSafetyStatus_Response_answer()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetSafetyStatus_Response_success answer(::ur_dashboard_msgs::srv::GetSafetyStatus_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GetSafetyStatus_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetSafetyStatus_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GetSafetyStatus_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GetSafetyStatus_Response_answer();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_SAFETY_STATUS__BUILDER_HPP_

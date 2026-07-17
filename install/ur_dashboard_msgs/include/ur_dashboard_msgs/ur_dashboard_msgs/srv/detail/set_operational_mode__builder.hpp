// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/SetOperationalMode.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__SET_OPERATIONAL_MODE__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__SET_OPERATIONAL_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/set_operational_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_SetOperationalMode_Request_operational_mode
{
public:
  Init_SetOperationalMode_Request_operational_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::srv::SetOperationalMode_Request operational_mode(::ur_dashboard_msgs::srv::SetOperationalMode_Request::_operational_mode_type arg)
  {
    msg_.operational_mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetOperationalMode_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::SetOperationalMode_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_SetOperationalMode_Request_operational_mode();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_SetOperationalMode_Response_success
{
public:
  explicit Init_SetOperationalMode_Response_success(::ur_dashboard_msgs::srv::SetOperationalMode_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::SetOperationalMode_Response success(::ur_dashboard_msgs::srv::SetOperationalMode_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetOperationalMode_Response msg_;
};

class Init_SetOperationalMode_Response_answer
{
public:
  Init_SetOperationalMode_Response_answer()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetOperationalMode_Response_success answer(::ur_dashboard_msgs::srv::SetOperationalMode_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_SetOperationalMode_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetOperationalMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::SetOperationalMode_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_SetOperationalMode_Response_answer();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__SET_OPERATIONAL_MODE__BUILDER_HPP_

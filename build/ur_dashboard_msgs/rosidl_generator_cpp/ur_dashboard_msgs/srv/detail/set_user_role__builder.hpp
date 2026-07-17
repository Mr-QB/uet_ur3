// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/SetUserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/set_user_role__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_SetUserRole_Request_user_role
{
public:
  Init_SetUserRole_Request_user_role()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::srv::SetUserRole_Request user_role(::ur_dashboard_msgs::srv::SetUserRole_Request::_user_role_type arg)
  {
    msg_.user_role = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetUserRole_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::SetUserRole_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_SetUserRole_Request_user_role();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_SetUserRole_Response_answer
{
public:
  explicit Init_SetUserRole_Response_answer(::ur_dashboard_msgs::srv::SetUserRole_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::SetUserRole_Response answer(::ur_dashboard_msgs::srv::SetUserRole_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetUserRole_Response msg_;
};

class Init_SetUserRole_Response_success
{
public:
  Init_SetUserRole_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetUserRole_Response_answer success(::ur_dashboard_msgs::srv::SetUserRole_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SetUserRole_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::SetUserRole_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::SetUserRole_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_SetUserRole_Response_success();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__SET_USER_ROLE__BUILDER_HPP_

// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:msg/UserRole.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/msg/detail/user_role__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace msg
{

namespace builder
{

class Init_UserRole_role
{
public:
  Init_UserRole_role()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::msg::UserRole role(::ur_dashboard_msgs::msg::UserRole::_role_type arg)
  {
    msg_.role = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::UserRole msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::msg::UserRole>()
{
  return ur_dashboard_msgs::msg::builder::Init_UserRole_role();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__USER_ROLE__BUILDER_HPP_

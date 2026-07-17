// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:msg/SafetyStatus.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/msg/detail/safety_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace msg
{

namespace builder
{

class Init_SafetyStatus_status
{
public:
  Init_SafetyStatus_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::msg::SafetyStatus status(::ur_dashboard_msgs::msg::SafetyStatus::_status_type arg)
  {
    msg_.status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::SafetyStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::msg::SafetyStatus>()
{
  return ur_dashboard_msgs::msg::builder::Init_SafetyStatus_status();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__SAFETY_STATUS__BUILDER_HPP_

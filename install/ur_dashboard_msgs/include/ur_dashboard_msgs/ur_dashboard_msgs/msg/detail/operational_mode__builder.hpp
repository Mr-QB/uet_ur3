// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:msg/OperationalMode.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/msg/detail/operational_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace msg
{

namespace builder
{

class Init_OperationalMode_mode
{
public:
  Init_OperationalMode_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::msg::OperationalMode mode(::ur_dashboard_msgs::msg::OperationalMode::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::OperationalMode msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::msg::OperationalMode>()
{
  return ur_dashboard_msgs::msg::builder::Init_OperationalMode_mode();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__OPERATIONAL_MODE__BUILDER_HPP_

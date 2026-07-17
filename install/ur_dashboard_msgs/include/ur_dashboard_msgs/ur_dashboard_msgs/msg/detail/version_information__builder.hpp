// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:msg/VersionInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/msg/detail/version_information__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace msg
{

namespace builder
{

class Init_VersionInformation_build
{
public:
  explicit Init_VersionInformation_build(::ur_dashboard_msgs::msg::VersionInformation & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::msg::VersionInformation build(::ur_dashboard_msgs::msg::VersionInformation::_build_type arg)
  {
    msg_.build = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::VersionInformation msg_;
};

class Init_VersionInformation_bugfix
{
public:
  explicit Init_VersionInformation_bugfix(::ur_dashboard_msgs::msg::VersionInformation & msg)
  : msg_(msg)
  {}
  Init_VersionInformation_build bugfix(::ur_dashboard_msgs::msg::VersionInformation::_bugfix_type arg)
  {
    msg_.bugfix = std::move(arg);
    return Init_VersionInformation_build(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::VersionInformation msg_;
};

class Init_VersionInformation_minor
{
public:
  explicit Init_VersionInformation_minor(::ur_dashboard_msgs::msg::VersionInformation & msg)
  : msg_(msg)
  {}
  Init_VersionInformation_bugfix minor(::ur_dashboard_msgs::msg::VersionInformation::_minor_type arg)
  {
    msg_.minor = std::move(arg);
    return Init_VersionInformation_bugfix(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::VersionInformation msg_;
};

class Init_VersionInformation_major
{
public:
  Init_VersionInformation_major()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_VersionInformation_minor major(::ur_dashboard_msgs::msg::VersionInformation::_major_type arg)
  {
    msg_.major = std::move(arg);
    return Init_VersionInformation_minor(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::VersionInformation msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::msg::VersionInformation>()
{
  return ur_dashboard_msgs::msg::builder::Init_VersionInformation_major();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__VERSION_INFORMATION__BUILDER_HPP_

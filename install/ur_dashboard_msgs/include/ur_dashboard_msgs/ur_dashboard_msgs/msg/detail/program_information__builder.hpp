// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:msg/ProgramInformation.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/msg/detail/program_information__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace msg
{

namespace builder
{

class Init_ProgramInformation_program_state
{
public:
  explicit Init_ProgramInformation_program_state(::ur_dashboard_msgs::msg::ProgramInformation & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::msg::ProgramInformation program_state(::ur_dashboard_msgs::msg::ProgramInformation::_program_state_type arg)
  {
    msg_.program_state = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

class Init_ProgramInformation_last_modified_date
{
public:
  explicit Init_ProgramInformation_last_modified_date(::ur_dashboard_msgs::msg::ProgramInformation & msg)
  : msg_(msg)
  {}
  Init_ProgramInformation_program_state last_modified_date(::ur_dashboard_msgs::msg::ProgramInformation::_last_modified_date_type arg)
  {
    msg_.last_modified_date = std::move(arg);
    return Init_ProgramInformation_program_state(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

class Init_ProgramInformation_last_saved_date
{
public:
  explicit Init_ProgramInformation_last_saved_date(::ur_dashboard_msgs::msg::ProgramInformation & msg)
  : msg_(msg)
  {}
  Init_ProgramInformation_last_modified_date last_saved_date(::ur_dashboard_msgs::msg::ProgramInformation::_last_saved_date_type arg)
  {
    msg_.last_saved_date = std::move(arg);
    return Init_ProgramInformation_last_modified_date(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

class Init_ProgramInformation_created_date
{
public:
  explicit Init_ProgramInformation_created_date(::ur_dashboard_msgs::msg::ProgramInformation & msg)
  : msg_(msg)
  {}
  Init_ProgramInformation_last_saved_date created_date(::ur_dashboard_msgs::msg::ProgramInformation::_created_date_type arg)
  {
    msg_.created_date = std::move(arg);
    return Init_ProgramInformation_last_saved_date(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

class Init_ProgramInformation_description
{
public:
  explicit Init_ProgramInformation_description(::ur_dashboard_msgs::msg::ProgramInformation & msg)
  : msg_(msg)
  {}
  Init_ProgramInformation_created_date description(::ur_dashboard_msgs::msg::ProgramInformation::_description_type arg)
  {
    msg_.description = std::move(arg);
    return Init_ProgramInformation_created_date(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

class Init_ProgramInformation_name
{
public:
  Init_ProgramInformation_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ProgramInformation_description name(::ur_dashboard_msgs::msg::ProgramInformation::_name_type arg)
  {
    msg_.name = std::move(arg);
    return Init_ProgramInformation_description(msg_);
  }

private:
  ::ur_dashboard_msgs::msg::ProgramInformation msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::msg::ProgramInformation>()
{
  return ur_dashboard_msgs::msg::builder::Init_ProgramInformation_name();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__MSG__DETAIL__PROGRAM_INFORMATION__BUILDER_HPP_

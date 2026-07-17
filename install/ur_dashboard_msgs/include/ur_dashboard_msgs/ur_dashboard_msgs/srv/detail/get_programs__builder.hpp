// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GetPrograms.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_PROGRAMS__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_PROGRAMS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/get_programs__struct.hpp"
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
auto build<::ur_dashboard_msgs::srv::GetPrograms_Request>()
{
  return ::ur_dashboard_msgs::srv::GetPrograms_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GetPrograms_Response_success
{
public:
  explicit Init_GetPrograms_Response_success(::ur_dashboard_msgs::srv::GetPrograms_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GetPrograms_Response success(::ur_dashboard_msgs::srv::GetPrograms_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetPrograms_Response msg_;
};

class Init_GetPrograms_Response_answer
{
public:
  explicit Init_GetPrograms_Response_answer(::ur_dashboard_msgs::srv::GetPrograms_Response & msg)
  : msg_(msg)
  {}
  Init_GetPrograms_Response_success answer(::ur_dashboard_msgs::srv::GetPrograms_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GetPrograms_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetPrograms_Response msg_;
};

class Init_GetPrograms_Response_programs
{
public:
  Init_GetPrograms_Response_programs()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetPrograms_Response_answer programs(::ur_dashboard_msgs::srv::GetPrograms_Response::_programs_type arg)
  {
    msg_.programs = std::move(arg);
    return Init_GetPrograms_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetPrograms_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GetPrograms_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GetPrograms_Response_programs();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_PROGRAMS__BUILDER_HPP_

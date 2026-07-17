// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GenerateSupportFile.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/generate_support_file__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GenerateSupportFile_Request_dir_path
{
public:
  Init_GenerateSupportFile_Request_dir_path()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Request dir_path(::ur_dashboard_msgs::srv::GenerateSupportFile_Request::_dir_path_type arg)
  {
    msg_.dir_path = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GenerateSupportFile_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_GenerateSupportFile_Request_dir_path();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GenerateSupportFile_Response_generated_file_name
{
public:
  explicit Init_GenerateSupportFile_Response_generated_file_name(::ur_dashboard_msgs::srv::GenerateSupportFile_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Response generated_file_name(::ur_dashboard_msgs::srv::GenerateSupportFile_Response::_generated_file_name_type arg)
  {
    msg_.generated_file_name = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Response msg_;
};

class Init_GenerateSupportFile_Response_answer
{
public:
  explicit Init_GenerateSupportFile_Response_answer(::ur_dashboard_msgs::srv::GenerateSupportFile_Response & msg)
  : msg_(msg)
  {}
  Init_GenerateSupportFile_Response_generated_file_name answer(::ur_dashboard_msgs::srv::GenerateSupportFile_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GenerateSupportFile_Response_generated_file_name(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Response msg_;
};

class Init_GenerateSupportFile_Response_success
{
public:
  Init_GenerateSupportFile_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GenerateSupportFile_Response_answer success(::ur_dashboard_msgs::srv::GenerateSupportFile_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GenerateSupportFile_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GenerateSupportFile_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GenerateSupportFile_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GenerateSupportFile_Response_success();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GENERATE_SUPPORT_FILE__BUILDER_HPP_

// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/UploadProgram.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/upload_program__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_UploadProgram_Request_file_path
{
public:
  Init_UploadProgram_Request_file_path()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur_dashboard_msgs::srv::UploadProgram_Request file_path(::ur_dashboard_msgs::srv::UploadProgram_Request::_file_path_type arg)
  {
    msg_.file_path = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::UploadProgram_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::UploadProgram_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_UploadProgram_Request_file_path();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_UploadProgram_Response_success
{
public:
  explicit Init_UploadProgram_Response_success(::ur_dashboard_msgs::srv::UploadProgram_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::UploadProgram_Response success(::ur_dashboard_msgs::srv::UploadProgram_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::UploadProgram_Response msg_;
};

class Init_UploadProgram_Response_program_name
{
public:
  explicit Init_UploadProgram_Response_program_name(::ur_dashboard_msgs::srv::UploadProgram_Response & msg)
  : msg_(msg)
  {}
  Init_UploadProgram_Response_success program_name(::ur_dashboard_msgs::srv::UploadProgram_Response::_program_name_type arg)
  {
    msg_.program_name = std::move(arg);
    return Init_UploadProgram_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::UploadProgram_Response msg_;
};

class Init_UploadProgram_Response_answer
{
public:
  Init_UploadProgram_Response_answer()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UploadProgram_Response_program_name answer(::ur_dashboard_msgs::srv::UploadProgram_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_UploadProgram_Response_program_name(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::UploadProgram_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::UploadProgram_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_UploadProgram_Response_answer();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__UPLOAD_PROGRAM__BUILDER_HPP_

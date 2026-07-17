// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/DownloadProgram.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__DOWNLOAD_PROGRAM__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__DOWNLOAD_PROGRAM__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/download_program__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_DownloadProgram_Request_target_path
{
public:
  explicit Init_DownloadProgram_Request_target_path(::ur_dashboard_msgs::srv::DownloadProgram_Request & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::DownloadProgram_Request target_path(::ur_dashboard_msgs::srv::DownloadProgram_Request::_target_path_type arg)
  {
    msg_.target_path = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::DownloadProgram_Request msg_;
};

class Init_DownloadProgram_Request_program_name
{
public:
  Init_DownloadProgram_Request_program_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DownloadProgram_Request_target_path program_name(::ur_dashboard_msgs::srv::DownloadProgram_Request::_program_name_type arg)
  {
    msg_.program_name = std::move(arg);
    return Init_DownloadProgram_Request_target_path(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::DownloadProgram_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::DownloadProgram_Request>()
{
  return ur_dashboard_msgs::srv::builder::Init_DownloadProgram_Request_program_name();
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_DownloadProgram_Response_success
{
public:
  explicit Init_DownloadProgram_Response_success(::ur_dashboard_msgs::srv::DownloadProgram_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::DownloadProgram_Response success(::ur_dashboard_msgs::srv::DownloadProgram_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::DownloadProgram_Response msg_;
};

class Init_DownloadProgram_Response_answer
{
public:
  Init_DownloadProgram_Response_answer()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DownloadProgram_Response_success answer(::ur_dashboard_msgs::srv::DownloadProgram_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_DownloadProgram_Response_success(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::DownloadProgram_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::DownloadProgram_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_DownloadProgram_Response_answer();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__DOWNLOAD_PROGRAM__BUILDER_HPP_

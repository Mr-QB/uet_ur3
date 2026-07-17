// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur_dashboard_msgs:srv/GetRobotModel.idl
// generated code does not contain a copyright notice

#ifndef UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__BUILDER_HPP_
#define UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur_dashboard_msgs/srv/detail/get_robot_model__struct.hpp"
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
auto build<::ur_dashboard_msgs::srv::GetRobotModel_Request>()
{
  return ::ur_dashboard_msgs::srv::GetRobotModel_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace ur_dashboard_msgs


namespace ur_dashboard_msgs
{

namespace srv
{

namespace builder
{

class Init_GetRobotModel_Response_robot_model
{
public:
  explicit Init_GetRobotModel_Response_robot_model(::ur_dashboard_msgs::srv::GetRobotModel_Response & msg)
  : msg_(msg)
  {}
  ::ur_dashboard_msgs::srv::GetRobotModel_Response robot_model(::ur_dashboard_msgs::srv::GetRobotModel_Response::_robot_model_type arg)
  {
    msg_.robot_model = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetRobotModel_Response msg_;
};

class Init_GetRobotModel_Response_answer
{
public:
  explicit Init_GetRobotModel_Response_answer(::ur_dashboard_msgs::srv::GetRobotModel_Response & msg)
  : msg_(msg)
  {}
  Init_GetRobotModel_Response_robot_model answer(::ur_dashboard_msgs::srv::GetRobotModel_Response::_answer_type arg)
  {
    msg_.answer = std::move(arg);
    return Init_GetRobotModel_Response_robot_model(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetRobotModel_Response msg_;
};

class Init_GetRobotModel_Response_success
{
public:
  Init_GetRobotModel_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetRobotModel_Response_answer success(::ur_dashboard_msgs::srv::GetRobotModel_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_GetRobotModel_Response_answer(msg_);
  }

private:
  ::ur_dashboard_msgs::srv::GetRobotModel_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur_dashboard_msgs::srv::GetRobotModel_Response>()
{
  return ur_dashboard_msgs::srv::builder::Init_GetRobotModel_Response_success();
}

}  // namespace ur_dashboard_msgs

#endif  // UR_DASHBOARD_MSGS__SRV__DETAIL__GET_ROBOT_MODEL__BUILDER_HPP_

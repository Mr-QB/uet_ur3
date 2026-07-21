// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ur3_moveit_control:action/UR3Control.idl
// generated code does not contain a copyright notice

#ifndef UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__BUILDER_HPP_
#define UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ur3_moveit_control/action/detail/ur3_control__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_Goal_cartesian_z_offset
{
public:
  explicit Init_UR3Control_Goal_cartesian_z_offset(::ur3_moveit_control::action::UR3Control_Goal & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_Goal cartesian_z_offset(::ur3_moveit_control::action::UR3Control_Goal::_cartesian_z_offset_type arg)
  {
    msg_.cartesian_z_offset = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Goal msg_;
};

class Init_UR3Control_Goal_pose_goal
{
public:
  explicit Init_UR3Control_Goal_pose_goal(::ur3_moveit_control::action::UR3Control_Goal & msg)
  : msg_(msg)
  {}
  Init_UR3Control_Goal_cartesian_z_offset pose_goal(::ur3_moveit_control::action::UR3Control_Goal::_pose_goal_type arg)
  {
    msg_.pose_goal = std::move(arg);
    return Init_UR3Control_Goal_cartesian_z_offset(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Goal msg_;
};

class Init_UR3Control_Goal_joint_goal
{
public:
  explicit Init_UR3Control_Goal_joint_goal(::ur3_moveit_control::action::UR3Control_Goal & msg)
  : msg_(msg)
  {}
  Init_UR3Control_Goal_pose_goal joint_goal(::ur3_moveit_control::action::UR3Control_Goal::_joint_goal_type arg)
  {
    msg_.joint_goal = std::move(arg);
    return Init_UR3Control_Goal_pose_goal(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Goal msg_;
};

class Init_UR3Control_Goal_command_type
{
public:
  Init_UR3Control_Goal_command_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_Goal_joint_goal command_type(::ur3_moveit_control::action::UR3Control_Goal::_command_type_type arg)
  {
    msg_.command_type = std::move(arg);
    return Init_UR3Control_Goal_joint_goal(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_Goal>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_Goal_command_type();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_Result_message
{
public:
  explicit Init_UR3Control_Result_message(::ur3_moveit_control::action::UR3Control_Result & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_Result message(::ur3_moveit_control::action::UR3Control_Result::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Result msg_;
};

class Init_UR3Control_Result_success
{
public:
  Init_UR3Control_Result_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_Result_message success(::ur3_moveit_control::action::UR3Control_Result::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_UR3Control_Result_message(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_Result>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_Result_success();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_Feedback_state
{
public:
  Init_UR3Control_Feedback_state()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur3_moveit_control::action::UR3Control_Feedback state(::ur3_moveit_control::action::UR3Control_Feedback::_state_type arg)
  {
    msg_.state = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_Feedback>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_Feedback_state();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_SendGoal_Request_goal
{
public:
  explicit Init_UR3Control_SendGoal_Request_goal(::ur3_moveit_control::action::UR3Control_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_SendGoal_Request goal(::ur3_moveit_control::action::UR3Control_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_SendGoal_Request msg_;
};

class Init_UR3Control_SendGoal_Request_goal_id
{
public:
  Init_UR3Control_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_SendGoal_Request_goal goal_id(::ur3_moveit_control::action::UR3Control_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_UR3Control_SendGoal_Request_goal(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_SendGoal_Request>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_SendGoal_Request_goal_id();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_SendGoal_Response_stamp
{
public:
  explicit Init_UR3Control_SendGoal_Response_stamp(::ur3_moveit_control::action::UR3Control_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_SendGoal_Response stamp(::ur3_moveit_control::action::UR3Control_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_SendGoal_Response msg_;
};

class Init_UR3Control_SendGoal_Response_accepted
{
public:
  Init_UR3Control_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_SendGoal_Response_stamp accepted(::ur3_moveit_control::action::UR3Control_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_UR3Control_SendGoal_Response_stamp(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_SendGoal_Response>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_SendGoal_Response_accepted();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_GetResult_Request_goal_id
{
public:
  Init_UR3Control_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ur3_moveit_control::action::UR3Control_GetResult_Request goal_id(::ur3_moveit_control::action::UR3Control_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_GetResult_Request>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_GetResult_Request_goal_id();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_GetResult_Response_result
{
public:
  explicit Init_UR3Control_GetResult_Response_result(::ur3_moveit_control::action::UR3Control_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_GetResult_Response result(::ur3_moveit_control::action::UR3Control_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_GetResult_Response msg_;
};

class Init_UR3Control_GetResult_Response_status
{
public:
  Init_UR3Control_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_GetResult_Response_result status(::ur3_moveit_control::action::UR3Control_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_UR3Control_GetResult_Response_result(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_GetResult_Response>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_GetResult_Response_status();
}

}  // namespace ur3_moveit_control


namespace ur3_moveit_control
{

namespace action
{

namespace builder
{

class Init_UR3Control_FeedbackMessage_feedback
{
public:
  explicit Init_UR3Control_FeedbackMessage_feedback(::ur3_moveit_control::action::UR3Control_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::ur3_moveit_control::action::UR3Control_FeedbackMessage feedback(::ur3_moveit_control::action::UR3Control_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_FeedbackMessage msg_;
};

class Init_UR3Control_FeedbackMessage_goal_id
{
public:
  Init_UR3Control_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_UR3Control_FeedbackMessage_feedback goal_id(::ur3_moveit_control::action::UR3Control_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_UR3Control_FeedbackMessage_feedback(msg_);
  }

private:
  ::ur3_moveit_control::action::UR3Control_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ur3_moveit_control::action::UR3Control_FeedbackMessage>()
{
  return ur3_moveit_control::action::builder::Init_UR3Control_FeedbackMessage_goal_id();
}

}  // namespace ur3_moveit_control

#endif  // UR3_MOVEIT_CONTROL__ACTION__DETAIL__UR3_CONTROL__BUILDER_HPP_

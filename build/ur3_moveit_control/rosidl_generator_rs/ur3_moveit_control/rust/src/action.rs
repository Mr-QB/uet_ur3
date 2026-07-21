
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ur3_moveit_control__action__UR3Control_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command_type: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_goal: sensor_msgs::msg::JointState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_goal: geometry_msgs::msg::PoseStamped,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cartesian_z_offset: f64,

}

impl UR3Control_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOVE_HOME: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOVE_JOINT: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MOVE_POSE: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ATTACH_AND_LIFT: u8 = 3;

}


impl Default for UR3Control_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Goal {
  type RmwMsg = super::action::rmw::UR3Control_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command_type: msg.command_type,
        joint_goal: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Owned(msg.joint_goal)).into_owned(),
        pose_goal: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose_goal)).into_owned(),
        cartesian_z_offset: msg.cartesian_z_offset,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      command_type: msg.command_type,
        joint_goal: sensor_msgs::msg::JointState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.joint_goal)).into_owned(),
        pose_goal: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose_goal)).into_owned(),
      cartesian_z_offset: msg.cartesian_z_offset,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command_type: msg.command_type,
      joint_goal: sensor_msgs::msg::JointState::from_rmw_message(msg.joint_goal),
      pose_goal: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose_goal),
      cartesian_z_offset: msg.cartesian_z_offset,
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for UR3Control_Result {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_Result::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Result {
  type RmwMsg = super::action::rmw::UR3Control_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: std::string::String,

}



impl Default for UR3Control_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Feedback {
  type RmwMsg = super::action::rmw::UR3Control_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: msg.state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: msg.state.to_string(),
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::UR3Control_Feedback,

}



impl Default for UR3Control_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_FeedbackMessage {
  type RmwMsg = super::action::rmw::UR3Control_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::UR3Control_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::UR3Control_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::UR3Control_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to ur3_moveit_control__action__UR3Control_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::UR3Control_Goal,

}



impl Default for UR3Control_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_SendGoal_Request {
  type RmwMsg = super::action::rmw::UR3Control_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::UR3Control_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::UR3Control_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::UR3Control_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for UR3Control_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_SendGoal_Response {
  type RmwMsg = super::action::rmw::UR3Control_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for UR3Control_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_GetResult_Request {
  type RmwMsg = super::action::rmw::UR3Control_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to ur3_moveit_control__action__UR3Control_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::UR3Control_Result,

}



impl Default for UR3Control_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::UR3Control_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UR3Control_GetResult_Response {
  type RmwMsg = super::action::rmw::UR3Control_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::UR3Control_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::UR3Control_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::UR3Control_Result::from_rmw_message(msg.result),
    }
  }
}






#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ur3_moveit_control__action__UR3Control_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct UR3Control_SendGoal;

impl rosidl_runtime_rs::Service for UR3Control_SendGoal {
    type Request = UR3Control_SendGoal_Request;
    type Response = UR3Control_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal() }
    }
}




#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ur3_moveit_control__action__UR3Control_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct UR3Control_GetResult;

impl rosidl_runtime_rs::Service for UR3Control_GetResult {
    type Request = UR3Control_GetResult_Request;
    type Response = UR3Control_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult() }
    }
}






#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__ur3_moveit_control__action__UR3Control() -> *const std::ffi::c_void;
}

// Corresponds to ur3_moveit_control__action__UR3Control
#[allow(missing_docs, non_camel_case_types)]
pub struct UR3Control;

impl rosidl_runtime_rs::Action for UR3Control {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = UR3Control_Goal;

  /// The result message defined in the action definition.
  type Result = UR3Control_Result;

  /// The feedback message defined in the action definition.
  type Feedback = UR3Control_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::UR3Control_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::UR3Control_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::UR3Control_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__ur3_moveit_control__action__UR3Control() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::UR3Control_Goal,
  ) -> super::action::rmw::UR3Control_SendGoal_Request {
   super::action::rmw::UR3Control_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::UR3Control_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::UR3Control_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::UR3Control_SendGoal_Response {
   super::action::rmw::UR3Control_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::UR3Control_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::UR3Control_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::UR3Control_Feedback,
  ) -> super::action::rmw::UR3Control_FeedbackMessage {
    let mut message = super::action::rmw::UR3Control_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::UR3Control_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::UR3Control_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::UR3Control_GetResult_Request {
   super::action::rmw::UR3Control_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::UR3Control_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::UR3Control_Result,
  ) -> super::action::rmw::UR3Control_GetResult_Response {
   super::action::rmw::UR3Control_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::UR3Control_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::UR3Control_Result,
  ) {
    (response.status, response.result)
  }
}



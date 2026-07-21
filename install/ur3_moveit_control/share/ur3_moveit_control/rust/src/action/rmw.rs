
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_Goal__init(msg: *mut UR3Control_Goal) -> bool;
    fn ur3_moveit_control__action__UR3Control_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Goal>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Goal>);
    fn ur3_moveit_control__action__UR3Control_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Goal>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command_type: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_goal: sensor_msgs::msg::rmw::JointState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_goal: geometry_msgs::msg::rmw::PoseStamped,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_Goal__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Goal() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Result() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_Result__init(msg: *mut UR3Control_Result) -> bool;
    fn ur3_moveit_control__action__UR3Control_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Result>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Result>);
    fn ur3_moveit_control__action__UR3Control_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Result>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for UR3Control_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_Result__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Result() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_Feedback__init(msg: *mut UR3Control_Feedback) -> bool;
    fn ur3_moveit_control__action__UR3Control_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Feedback>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Feedback>);
    fn ur3_moveit_control__action__UR3Control_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_Feedback>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: rosidl_runtime_rs::String,

}



impl Default for UR3Control_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_Feedback() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_FeedbackMessage__init(msg: *mut UR3Control_FeedbackMessage) -> bool;
    fn ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_FeedbackMessage>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_FeedbackMessage>);
    fn ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_FeedbackMessage>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::UR3Control_Feedback,

}



impl Default for UR3Control_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_FeedbackMessage() }
  }
}




#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_SendGoal_Request__init(msg: *mut UR3Control_SendGoal_Request) -> bool;
    fn ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Request>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Request>);
    fn ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Request>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::UR3Control_Goal,

}



impl Default for UR3Control_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal_Request() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_SendGoal_Response__init(msg: *mut UR3Control_SendGoal_Response) -> bool;
    fn ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Response>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Response>);
    fn ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_SendGoal_Response>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for UR3Control_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_SendGoal_Response() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_GetResult_Request__init(msg: *mut UR3Control_GetResult_Request) -> bool;
    fn ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Request>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Request>);
    fn ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Request>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for UR3Control_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult_Request() }
  }
}


#[link(name = "ur3_moveit_control__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur3_moveit_control__rosidl_generator_c")]
extern "C" {
    fn ur3_moveit_control__action__UR3Control_GetResult_Response__init(msg: *mut UR3Control_GetResult_Response) -> bool;
    fn ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Response>, size: usize) -> bool;
    fn ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Response>);
    fn ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UR3Control_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UR3Control_GetResult_Response>) -> bool;
}

// Corresponds to ur3_moveit_control__action__UR3Control_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UR3Control_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::UR3Control_Result,

}



impl Default for UR3Control_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur3_moveit_control__action__UR3Control_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ur3_moveit_control__action__UR3Control_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UR3Control_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur3_moveit_control__action__UR3Control_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UR3Control_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UR3Control_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur3_moveit_control/action/UR3Control_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur3_moveit_control__action__UR3Control_GetResult_Response() }
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




#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_Goal__init(msg: *mut SetMode_Goal) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Goal>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Goal>);
    fn ur_dashboard_msgs__action__SetMode_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_Goal>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_Goal {
    /// Target modes can be one of
    /// - 3: ROBOT_MODE_POWER_OFF
    /// - 5: ROBOT_MODE_IDLE
    /// - 7: ROBOT_MODE_RUNNING
    pub target_robot_mode: i8,

    /// Stop program execution before restoring the target mode. Can be used together with 'play_program'.
    pub stop_program: bool,

    /// Play the currently loaded program after target mode is reached.#
    /// NOTE: Requesting mode RUNNING in combination with this will make the robot continue the motion it
    /// was doing before. This might probably lead into the same problem (protective stop, EM-Stop due to
    /// faulty motion, etc.) If you want to be safe, set the 'stop_program' flag below and manually play
    /// the program after robot state is returned to normal.
    /// This flag will only be used when requesting mode RUNNING
    pub play_program: bool,

}



impl Default for SetMode_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_Goal__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Goal() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Result() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_Result__init(msg: *mut SetMode_Result) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Result>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Result>);
    fn ur_dashboard_msgs__action__SetMode_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_Result>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SetMode_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_Result__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Result() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_Feedback__init(msg: *mut SetMode_Feedback) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Feedback>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_Feedback>);
    fn ur_dashboard_msgs__action__SetMode_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_Feedback>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_robot_mode: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_safety_mode: i8,

}



impl Default for SetMode_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_Feedback() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_FeedbackMessage__init(msg: *mut SetMode_FeedbackMessage) -> bool;
    fn ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_FeedbackMessage>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_FeedbackMessage>);
    fn ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_FeedbackMessage>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::SetMode_Feedback,

}



impl Default for SetMode_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_FeedbackMessage() }
  }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Request__init(msg: *mut SetMode_SendGoal_Request) -> bool;
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Request>);
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::SetMode_Goal,

}



impl Default for SetMode_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Response__init(msg: *mut SetMode_SendGoal_Response) -> bool;
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Response>);
    fn ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_SendGoal_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for SetMode_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_GetResult_Request__init(msg: *mut SetMode_GetResult_Request) -> bool;
    fn ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Request>);
    fn ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for SetMode_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__action__SetMode_GetResult_Response__init(msg: *mut SetMode_GetResult_Response) -> bool;
    fn ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Response>);
    fn ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMode_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMode_GetResult_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMode_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::SetMode_Result,

}



impl Default for SetMode_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__action__SetMode_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__action__SetMode_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMode_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__action__SetMode_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMode_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMode_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/action/SetMode_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult_Response() }
  }
}






#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct SetMode_SendGoal;

impl rosidl_runtime_rs::Service for SetMode_SendGoal {
    type Request = SetMode_SendGoal_Request;
    type Response = SetMode_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__action__SetMode_SendGoal() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__action__SetMode_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct SetMode_GetResult;

impl rosidl_runtime_rs::Service for SetMode_GetResult {
    type Request = SetMode_GetResult_Request;
    type Response = SetMode_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__action__SetMode_GetResult() }
    }
}



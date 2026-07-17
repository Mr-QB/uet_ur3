#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__AddToLog_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__AddToLog_Request__init(msg: *mut AddToLog_Request) -> bool;
    fn ur_dashboard_msgs__srv__AddToLog_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__AddToLog_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Request>);
    fn ur_dashboard_msgs__srv__AddToLog_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddToLog_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__AddToLog_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddToLog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for AddToLog_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__AddToLog_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__AddToLog_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddToLog_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddToLog_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddToLog_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/AddToLog_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__AddToLog_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__AddToLog_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__AddToLog_Response__init(msg: *mut AddToLog_Response) -> bool;
    fn ur_dashboard_msgs__srv__AddToLog_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__AddToLog_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Response>);
    fn ur_dashboard_msgs__srv__AddToLog_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddToLog_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddToLog_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__AddToLog_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddToLog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for AddToLog_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__AddToLog_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__AddToLog_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddToLog_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__AddToLog_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddToLog_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddToLog_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/AddToLog_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__AddToLog_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__DownloadProgram_Request__init(msg: *mut DownloadProgram_Request) -> bool;
    fn ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Request>);
    fn ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DownloadProgram_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__DownloadProgram_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DownloadProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_path: rosidl_runtime_rs::String,

}



impl Default for DownloadProgram_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__DownloadProgram_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__DownloadProgram_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DownloadProgram_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DownloadProgram_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DownloadProgram_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/DownloadProgram_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__DownloadProgram_Response__init(msg: *mut DownloadProgram_Response) -> bool;
    fn ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Response>);
    fn ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DownloadProgram_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DownloadProgram_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__DownloadProgram_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DownloadProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for DownloadProgram_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__DownloadProgram_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__DownloadProgram_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DownloadProgram_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__DownloadProgram_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DownloadProgram_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DownloadProgram_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/DownloadProgram_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Request__init(msg: *mut GetLoadedProgram_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Request>);
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLoadedProgram_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetLoadedProgram_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLoadedProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetLoadedProgram_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetLoadedProgram_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetLoadedProgram_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLoadedProgram_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLoadedProgram_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLoadedProgram_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetLoadedProgram_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Response__init(msg: *mut GetLoadedProgram_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Response>);
    fn ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetLoadedProgram_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetLoadedProgram_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetLoadedProgram_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLoadedProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetLoadedProgram_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetLoadedProgram_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetLoadedProgram_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetLoadedProgram_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetLoadedProgram_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetLoadedProgram_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetLoadedProgram_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetLoadedProgram_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPrograms_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetPrograms_Request__init(msg: *mut GetPrograms_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Request>);
    fn ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPrograms_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetPrograms_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPrograms_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPrograms_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetPrograms_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetPrograms_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPrograms_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPrograms_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPrograms_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetPrograms_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPrograms_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPrograms_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetPrograms_Response__init(msg: *mut GetPrograms_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Response>);
    fn ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPrograms_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPrograms_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetPrograms_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPrograms_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub programs: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ProgramInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetPrograms_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetPrograms_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetPrograms_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPrograms_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPrograms_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPrograms_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPrograms_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetPrograms_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPrograms_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetProgramState_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetProgramState_Request__init(msg: *mut GetProgramState_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Request>);
    fn ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetProgramState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetProgramState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetProgramState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetProgramState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetProgramState_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetProgramState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetProgramState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetProgramState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetProgramState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetProgramState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetProgramState_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetProgramState_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetProgramState_Response__init(msg: *mut GetProgramState_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Response>);
    fn ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetProgramState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetProgramState_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetProgramState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetProgramState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::super::msg::rmw::ProgramState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetProgramState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetProgramState_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetProgramState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetProgramState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetProgramState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetProgramState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetProgramState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetProgramState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetProgramState_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetRobotMode_Request__init(msg: *mut GetRobotMode_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Request>);
    fn ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetRobotMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetRobotMode_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetRobotMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetRobotMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetRobotMode_Response__init(msg: *mut GetRobotMode_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Response>);
    fn ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotMode_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: super::super::msg::rmw::RobotMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetRobotMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetRobotMode_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetRobotMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetRobotMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSafetyMode_Request__init(msg: *mut GetSafetyMode_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Request>);
    fn ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSafetyMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSafetyMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSafetyMode_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSafetyMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSafetyMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSafetyMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSafetyMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSafetyMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSafetyMode_Response__init(msg: *mut GetSafetyMode_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Response>);
    fn ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSafetyMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSafetyMode_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_mode: super::super::msg::rmw::SafetyMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetSafetyMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSafetyMode_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSafetyMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSafetyMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSafetyMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSafetyMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSafetyMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsProgramRunning_Request__init(msg: *mut IsProgramRunning_Request) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Request>);
    fn ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsProgramRunning_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramRunning_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramRunning_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsProgramRunning_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsProgramRunning_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsProgramRunning_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsProgramRunning_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsProgramRunning_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsProgramRunning_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsProgramRunning_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsProgramRunning_Response__init(msg: *mut IsProgramRunning_Response) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Response>);
    fn ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsProgramRunning_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<IsProgramRunning_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramRunning_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramRunning_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

    /// is a program running?
    pub program_running: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsProgramRunning_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsProgramRunning_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsProgramRunning_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsProgramRunning_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramRunning_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsProgramRunning_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsProgramRunning_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsProgramRunning_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsProgramSaved_Request__init(msg: *mut IsProgramSaved_Request) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Request>);
    fn ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsProgramSaved_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramSaved_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramSaved_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsProgramSaved_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsProgramSaved_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsProgramSaved_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsProgramSaved_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsProgramSaved_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsProgramSaved_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsProgramSaved_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsProgramSaved_Response__init(msg: *mut IsProgramSaved_Response) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Response>);
    fn ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsProgramSaved_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<IsProgramSaved_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramSaved_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramSaved_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: rosidl_runtime_rs::String,

    /// is the current program saved?
    pub program_saved: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsProgramSaved_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsProgramSaved_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsProgramSaved_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsProgramSaved_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsProgramSaved_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsProgramSaved_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsProgramSaved_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsProgramSaved_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Load_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__Load_Request__init(msg: *mut Load_Request) -> bool;
    fn ur_dashboard_msgs__srv__Load_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Load_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__Load_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Load_Request>);
    fn ur_dashboard_msgs__srv__Load_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Load_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Load_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__Load_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Load_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for Load_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__Load_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__Load_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Load_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Load_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Load_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/Load_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Load_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Load_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__Load_Response__init(msg: *mut Load_Response) -> bool;
    fn ur_dashboard_msgs__srv__Load_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Load_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__Load_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Load_Response>);
    fn ur_dashboard_msgs__srv__Load_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Load_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Load_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__Load_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Load_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for Load_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__Load_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__Load_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Load_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Load_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Load_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Load_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/Load_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Load_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Popup_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__Popup_Request__init(msg: *mut Popup_Request) -> bool;
    fn ur_dashboard_msgs__srv__Popup_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Popup_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__Popup_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Popup_Request>);
    fn ur_dashboard_msgs__srv__Popup_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Popup_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Popup_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__Popup_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Popup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Popup_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__Popup_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__Popup_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Popup_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Popup_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Popup_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/Popup_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Popup_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Popup_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__Popup_Response__init(msg: *mut Popup_Response) -> bool;
    fn ur_dashboard_msgs__srv__Popup_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Popup_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__Popup_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Popup_Response>);
    fn ur_dashboard_msgs__srv__Popup_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Popup_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Popup_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__Popup_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Popup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for Popup_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__Popup_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__Popup_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Popup_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__Popup_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Popup_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Popup_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/Popup_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__Popup_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__RawRequest_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__RawRequest_Request__init(msg: *mut RawRequest_Request) -> bool;
    fn ur_dashboard_msgs__srv__RawRequest_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__RawRequest_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Request>);
    fn ur_dashboard_msgs__srv__RawRequest_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RawRequest_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__RawRequest_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawRequest_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub query: rosidl_runtime_rs::String,

}



impl Default for RawRequest_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__RawRequest_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__RawRequest_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RawRequest_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RawRequest_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RawRequest_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/RawRequest_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__RawRequest_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__RawRequest_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__RawRequest_Response__init(msg: *mut RawRequest_Response) -> bool;
    fn ur_dashboard_msgs__srv__RawRequest_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__RawRequest_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Response>);
    fn ur_dashboard_msgs__srv__RawRequest_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RawRequest_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RawRequest_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__RawRequest_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawRequest_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

}



impl Default for RawRequest_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__RawRequest_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__RawRequest_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RawRequest_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__RawRequest_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RawRequest_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RawRequest_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/RawRequest_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__RawRequest_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Request__init(msg: *mut IsInRemoteControl_Request) -> bool;
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Request>);
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsInRemoteControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsInRemoteControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsInRemoteControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsInRemoteControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsInRemoteControl_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsInRemoteControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsInRemoteControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsInRemoteControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsInRemoteControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsInRemoteControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Response__init(msg: *mut IsInRemoteControl_Response) -> bool;
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Response>);
    fn ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IsInRemoteControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<IsInRemoteControl_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__IsInRemoteControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsInRemoteControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

    /// is the robot in remote control?
    pub remote_control: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsInRemoteControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__IsInRemoteControl_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__IsInRemoteControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IsInRemoteControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__IsInRemoteControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IsInRemoteControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IsInRemoteControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/IsInRemoteControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__UploadProgram_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__UploadProgram_Request__init(msg: *mut UploadProgram_Request) -> bool;
    fn ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Request>);
    fn ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadProgram_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__UploadProgram_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: rosidl_runtime_rs::String,

}



impl Default for UploadProgram_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__UploadProgram_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__UploadProgram_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadProgram_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadProgram_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadProgram_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/UploadProgram_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__UploadProgram_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__UploadProgram_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__UploadProgram_Response__init(msg: *mut UploadProgram_Response) -> bool;
    fn ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Response>);
    fn ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UploadProgram_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<UploadProgram_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__UploadProgram_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for UploadProgram_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__UploadProgram_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__UploadProgram_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UploadProgram_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__UploadProgram_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UploadProgram_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UploadProgram_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/UploadProgram_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__UploadProgram_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__init(msg: *mut GetPolyScopeVersion_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Request>);
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetPolyScopeVersion_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPolyScopeVersion_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPolyScopeVersion_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPolyScopeVersion_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPolyScopeVersion_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPolyScopeVersion_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetPolyScopeVersion_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__init(msg: *mut GetPolyScopeVersion_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Response>);
    fn ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPolyScopeVersion_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetPolyScopeVersion_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPolyScopeVersion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: super::super::msg::rmw::VersionInformation,

}



impl Default for GetPolyScopeVersion_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPolyScopeVersion_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetPolyScopeVersion_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPolyScopeVersion_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPolyScopeVersion_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetPolyScopeVersion_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSerialNumber_Request__init(msg: *mut GetSerialNumber_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Request>);
    fn ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSerialNumber_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSerialNumber_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSerialNumber_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSerialNumber_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSerialNumber_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSerialNumber_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSerialNumber_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSerialNumber_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSerialNumber_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSerialNumber_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSerialNumber_Response__init(msg: *mut GetSerialNumber_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Response>);
    fn ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSerialNumber_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSerialNumber_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSerialNumber_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSerialNumber_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetSerialNumber_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSerialNumber_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSerialNumber_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSerialNumber_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSerialNumber_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSerialNumber_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSerialNumber_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSerialNumber_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetUserRole_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetUserRole_Request__init(msg: *mut GetUserRole_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Request>);
    fn ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetUserRole_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetUserRole_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetUserRole_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetUserRole_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetUserRole_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetUserRole_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetUserRole_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetUserRole_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetUserRole_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetUserRole_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetUserRole_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetUserRole_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetUserRole_Response__init(msg: *mut GetUserRole_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Response>);
    fn ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetUserRole_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetUserRole_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetUserRole_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetUserRole_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user_role: super::super::msg::rmw::UserRole,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetUserRole_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetUserRole_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetUserRole_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetUserRole_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetUserRole_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetUserRole_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetUserRole_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetUserRole_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetUserRole_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetUserRole_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__SetUserRole_Request__init(msg: *mut SetUserRole_Request) -> bool;
    fn ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Request>);
    fn ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetUserRole_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__SetUserRole_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUserRole_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user_role: super::super::msg::rmw::UserRole,

}



impl Default for SetUserRole_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__SetUserRole_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__SetUserRole_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetUserRole_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetUserRole_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetUserRole_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/SetUserRole_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetUserRole_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetUserRole_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__SetUserRole_Response__init(msg: *mut SetUserRole_Response) -> bool;
    fn ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Response>);
    fn ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetUserRole_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetUserRole_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__SetUserRole_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUserRole_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,

}



impl Default for SetUserRole_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__SetUserRole_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__SetUserRole_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetUserRole_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetUserRole_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetUserRole_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetUserRole_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/SetUserRole_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetUserRole_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__SetOperationalMode_Request__init(msg: *mut SetOperationalMode_Request) -> bool;
    fn ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Request>);
    fn ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOperationalMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__SetOperationalMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOperationalMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub operational_mode: super::super::msg::rmw::OperationalMode,

}



impl Default for SetOperationalMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__SetOperationalMode_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__SetOperationalMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOperationalMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOperationalMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOperationalMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/SetOperationalMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__SetOperationalMode_Response__init(msg: *mut SetOperationalMode_Response) -> bool;
    fn ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Response>);
    fn ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOperationalMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOperationalMode_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__SetOperationalMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOperationalMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetOperationalMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__SetOperationalMode_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__SetOperationalMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOperationalMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__SetOperationalMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOperationalMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOperationalMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/SetOperationalMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetOperationalMode_Request__init(msg: *mut GetOperationalMode_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Request>);
    fn ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOperationalMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetOperationalMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOperationalMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOperationalMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetOperationalMode_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetOperationalMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOperationalMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOperationalMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOperationalMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetOperationalMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetOperationalMode_Response__init(msg: *mut GetOperationalMode_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Response>);
    fn ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOperationalMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOperationalMode_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetOperationalMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOperationalMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operational_mode: super::super::msg::rmw::OperationalMode,

}



impl Default for GetOperationalMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetOperationalMode_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetOperationalMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOperationalMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetOperationalMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOperationalMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOperationalMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetOperationalMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetRobotModel_Request__init(msg: *mut GetRobotModel_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Request>);
    fn ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotModel_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotModel_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotModel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetRobotModel_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetRobotModel_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetRobotModel_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotModel_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotModel_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotModel_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetRobotModel_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetRobotModel_Response__init(msg: *mut GetRobotModel_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Response>);
    fn ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetRobotModel_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetRobotModel_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotModel_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotModel_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_model: rosidl_runtime_rs::String,

}



impl Default for GetRobotModel_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetRobotModel_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetRobotModel_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetRobotModel_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetRobotModel_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetRobotModel_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetRobotModel_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetRobotModel_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Request__init(msg: *mut GetSafetyStatus_Request) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Request>);
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSafetyStatus_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyStatus_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSafetyStatus_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSafetyStatus_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSafetyStatus_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSafetyStatus_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSafetyStatus_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSafetyStatus_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSafetyStatus_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Response__init(msg: *mut GetSafetyStatus_Response) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Response>);
    fn ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetSafetyStatus_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetSafetyStatus_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyStatus_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_status: super::super::msg::rmw::SafetyStatus,

}



impl Default for GetSafetyStatus_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GetSafetyStatus_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GetSafetyStatus_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetSafetyStatus_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GetSafetyStatus_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetSafetyStatus_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetSafetyStatus_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GetSafetyStatus_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Request__init(msg: *mut GenerateFlightReport_Request) -> bool;
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Request>);
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GenerateFlightReport_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateFlightReport_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateFlightReport_Request {
    /// Same default as in the dashboard server
    pub report_type: rosidl_runtime_rs::String,

}

impl GenerateFlightReport_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTROLLER: &'static str = "controller";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SOFTWARE: &'static str = "software";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SYSTEM: &'static str = "system";

}


impl Default for GenerateFlightReport_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GenerateFlightReport_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GenerateFlightReport_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GenerateFlightReport_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GenerateFlightReport_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GenerateFlightReport_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GenerateFlightReport_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Response__init(msg: *mut GenerateFlightReport_Response) -> bool;
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Response>);
    fn ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GenerateFlightReport_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GenerateFlightReport_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateFlightReport_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateFlightReport_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub report_id: rosidl_runtime_rs::String,

}



impl Default for GenerateFlightReport_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GenerateFlightReport_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GenerateFlightReport_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GenerateFlightReport_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateFlightReport_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GenerateFlightReport_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GenerateFlightReport_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GenerateFlightReport_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport_Response() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile_Request() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Request__init(msg: *mut GenerateSupportFile_Request) -> bool;
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Request>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Request>);
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GenerateSupportFile_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Request>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateSupportFile_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateSupportFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dir_path: rosidl_runtime_rs::String,

}



impl Default for GenerateSupportFile_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GenerateSupportFile_Request__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GenerateSupportFile_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GenerateSupportFile_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GenerateSupportFile_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GenerateSupportFile_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GenerateSupportFile_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile_Request() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile_Response() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Response__init(msg: *mut GenerateSupportFile_Response) -> bool;
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Response>, size: usize) -> bool;
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Response>);
    fn ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GenerateSupportFile_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GenerateSupportFile_Response>) -> bool;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateSupportFile_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateSupportFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub generated_file_name: rosidl_runtime_rs::String,

}



impl Default for GenerateSupportFile_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__srv__GenerateSupportFile_Response__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__srv__GenerateSupportFile_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GenerateSupportFile_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__srv__GenerateSupportFile_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GenerateSupportFile_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GenerateSupportFile_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/srv/GenerateSupportFile_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile_Response() }
  }
}






#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__AddToLog() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__AddToLog
#[allow(missing_docs, non_camel_case_types)]
pub struct AddToLog;

impl rosidl_runtime_rs::Service for AddToLog {
    type Request = AddToLog_Request;
    type Response = AddToLog_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__AddToLog() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__DownloadProgram
#[allow(missing_docs, non_camel_case_types)]
pub struct DownloadProgram;

impl rosidl_runtime_rs::Service for DownloadProgram {
    type Request = DownloadProgram_Request;
    type Response = DownloadProgram_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__DownloadProgram() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetLoadedProgram
#[allow(missing_docs, non_camel_case_types)]
pub struct GetLoadedProgram;

impl rosidl_runtime_rs::Service for GetLoadedProgram {
    type Request = GetLoadedProgram_Request;
    type Response = GetLoadedProgram_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetLoadedProgram() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetPrograms() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetPrograms
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPrograms;

impl rosidl_runtime_rs::Service for GetPrograms {
    type Request = GetPrograms_Request;
    type Response = GetPrograms_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetPrograms() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetProgramState() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetProgramState
#[allow(missing_docs, non_camel_case_types)]
pub struct GetProgramState;

impl rosidl_runtime_rs::Service for GetProgramState {
    type Request = GetProgramState_Request;
    type Response = GetProgramState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetProgramState() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotMode
#[allow(missing_docs, non_camel_case_types)]
pub struct GetRobotMode;

impl rosidl_runtime_rs::Service for GetRobotMode {
    type Request = GetRobotMode_Request;
    type Response = GetRobotMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetRobotMode() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyMode
#[allow(missing_docs, non_camel_case_types)]
pub struct GetSafetyMode;

impl rosidl_runtime_rs::Service for GetSafetyMode {
    type Request = GetSafetyMode_Request;
    type Response = GetSafetyMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSafetyMode() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramRunning
#[allow(missing_docs, non_camel_case_types)]
pub struct IsProgramRunning;

impl rosidl_runtime_rs::Service for IsProgramRunning {
    type Request = IsProgramRunning_Request;
    type Response = IsProgramRunning_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsProgramRunning() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__IsProgramSaved
#[allow(missing_docs, non_camel_case_types)]
pub struct IsProgramSaved;

impl rosidl_runtime_rs::Service for IsProgramSaved {
    type Request = IsProgramSaved_Request;
    type Response = IsProgramSaved_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsProgramSaved() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__Load() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__Load
#[allow(missing_docs, non_camel_case_types)]
pub struct Load;

impl rosidl_runtime_rs::Service for Load {
    type Request = Load_Request;
    type Response = Load_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__Load() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__Popup() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__Popup
#[allow(missing_docs, non_camel_case_types)]
pub struct Popup;

impl rosidl_runtime_rs::Service for Popup {
    type Request = Popup_Request;
    type Response = Popup_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__Popup() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__RawRequest() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__RawRequest
#[allow(missing_docs, non_camel_case_types)]
pub struct RawRequest;

impl rosidl_runtime_rs::Service for RawRequest {
    type Request = RawRequest_Request;
    type Response = RawRequest_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__RawRequest() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__IsInRemoteControl
#[allow(missing_docs, non_camel_case_types)]
pub struct IsInRemoteControl;

impl rosidl_runtime_rs::Service for IsInRemoteControl {
    type Request = IsInRemoteControl_Request;
    type Response = IsInRemoteControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__IsInRemoteControl() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__UploadProgram() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__UploadProgram
#[allow(missing_docs, non_camel_case_types)]
pub struct UploadProgram;

impl rosidl_runtime_rs::Service for UploadProgram {
    type Request = UploadProgram_Request;
    type Response = UploadProgram_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__UploadProgram() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetPolyScopeVersion
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPolyScopeVersion;

impl rosidl_runtime_rs::Service for GetPolyScopeVersion {
    type Request = GetPolyScopeVersion_Request;
    type Response = GetPolyScopeVersion_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetPolyScopeVersion() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetSerialNumber
#[allow(missing_docs, non_camel_case_types)]
pub struct GetSerialNumber;

impl rosidl_runtime_rs::Service for GetSerialNumber {
    type Request = GetSerialNumber_Request;
    type Response = GetSerialNumber_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSerialNumber() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetUserRole() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetUserRole
#[allow(missing_docs, non_camel_case_types)]
pub struct GetUserRole;

impl rosidl_runtime_rs::Service for GetUserRole {
    type Request = GetUserRole_Request;
    type Response = GetUserRole_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetUserRole() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__SetUserRole() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__SetUserRole
#[allow(missing_docs, non_camel_case_types)]
pub struct SetUserRole;

impl rosidl_runtime_rs::Service for SetUserRole {
    type Request = SetUserRole_Request;
    type Response = SetUserRole_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__SetUserRole() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__SetOperationalMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOperationalMode;

impl rosidl_runtime_rs::Service for SetOperationalMode {
    type Request = SetOperationalMode_Request;
    type Response = SetOperationalMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__SetOperationalMode() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetOperationalMode
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOperationalMode;

impl rosidl_runtime_rs::Service for GetOperationalMode {
    type Request = GetOperationalMode_Request;
    type Response = GetOperationalMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetOperationalMode() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetRobotModel
#[allow(missing_docs, non_camel_case_types)]
pub struct GetRobotModel;

impl rosidl_runtime_rs::Service for GetRobotModel {
    type Request = GetRobotModel_Request;
    type Response = GetRobotModel_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetRobotModel() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GetSafetyStatus
#[allow(missing_docs, non_camel_case_types)]
pub struct GetSafetyStatus;

impl rosidl_runtime_rs::Service for GetSafetyStatus {
    type Request = GetSafetyStatus_Request;
    type Response = GetSafetyStatus_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GetSafetyStatus() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateFlightReport
#[allow(missing_docs, non_camel_case_types)]
pub struct GenerateFlightReport;

impl rosidl_runtime_rs::Service for GenerateFlightReport {
    type Request = GenerateFlightReport_Request;
    type Response = GenerateFlightReport_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GenerateFlightReport() }
    }
}




#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile() -> *const std::ffi::c_void;
}

// Corresponds to ur_dashboard_msgs__srv__GenerateSupportFile
#[allow(missing_docs, non_camel_case_types)]
pub struct GenerateSupportFile;

impl rosidl_runtime_rs::Service for GenerateSupportFile {
    type Request = GenerateSupportFile_Request;
    type Response = GenerateSupportFile_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ur_dashboard_msgs__srv__GenerateSupportFile() }
    }
}



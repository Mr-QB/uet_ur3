#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__ProgramInformation() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__ProgramInformation__init(msg: *mut ProgramInformation) -> bool;
    fn ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ProgramInformation>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ProgramInformation>);
    fn ur_dashboard_msgs__msg__ProgramInformation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ProgramInformation>, out_seq: *mut rosidl_runtime_rs::Sequence<ProgramInformation>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__ProgramInformation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProgramInformation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub created_date: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub last_saved_date: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub last_modified_date: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_state: rosidl_runtime_rs::String,

}

impl ProgramInformation {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROGRAM_STATE_DRAFT: &'static str = "DRAFT";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROGRAM_STATE_FINAL: &'static str = "FINAL";

}


impl Default for ProgramInformation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__ProgramInformation__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__ProgramInformation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ProgramInformation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramInformation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramInformation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramInformation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ProgramInformation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ProgramInformation where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/ProgramInformation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__ProgramInformation() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__ProgramState() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__ProgramState__init(msg: *mut ProgramState) -> bool;
    fn ur_dashboard_msgs__msg__ProgramState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ProgramState>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__ProgramState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ProgramState>);
    fn ur_dashboard_msgs__msg__ProgramState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ProgramState>, out_seq: *mut rosidl_runtime_rs::Sequence<ProgramState>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__ProgramState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProgramState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: rosidl_runtime_rs::String,

}

impl ProgramState {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STOPPED: &'static str = "STOPPED";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PLAYING: &'static str = "PLAYING";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PAUSED: &'static str = "PAUSED";

}


impl Default for ProgramState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__ProgramState__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__ProgramState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ProgramState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__ProgramState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ProgramState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ProgramState where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/ProgramState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__ProgramState() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__RobotMode() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__RobotMode__init(msg: *mut RobotMode) -> bool;
    fn ur_dashboard_msgs__msg__RobotMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__RobotMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode>);
    fn ur_dashboard_msgs__msg__RobotMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__RobotMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i8,

}

impl RobotMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NO_CONTROLLER: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DISCONNECTED: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONFIRM_SAFETY: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BOOTING: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const POWER_OFF: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const POWER_ON: i8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IDLE: i8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BACKDRIVE: i8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RUNNING: i8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UPDATING_FIRMWARE: i8 = 8;

}


impl Default for RobotMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__RobotMode__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__RobotMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__RobotMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__RobotMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__RobotMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/RobotMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__RobotMode() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__SafetyMode() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__SafetyMode__init(msg: *mut SafetyMode) -> bool;
    fn ur_dashboard_msgs__msg__SafetyMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyMode>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__SafetyMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyMode>);
    fn ur_dashboard_msgs__msg__SafetyMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyMode>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyMode>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__SafetyMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,

}

impl SafetyMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NORMAL: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const REDUCED: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROTECTIVE_STOP: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RECOVERY: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SAFEGUARD_STOP: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SYSTEM_EMERGENCY_STOP: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROBOT_EMERGENCY_STOP: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const VIOLATION: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FAULT: u8 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const VALIDATE_JOINT_ID: u8 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNDEFINED_SAFETY_MODE: u8 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AUTOMATIC_MODE_SAFEGUARD_STOP: u8 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SYSTEM_THREE_POSITION_ENABLING_STOP: u8 = 13;

}


impl Default for SafetyMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__SafetyMode__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__SafetyMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyMode where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/SafetyMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__SafetyMode() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__OperationalMode() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__OperationalMode__init(msg: *mut OperationalMode) -> bool;
    fn ur_dashboard_msgs__msg__OperationalMode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OperationalMode>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__OperationalMode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OperationalMode>);
    fn ur_dashboard_msgs__msg__OperationalMode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OperationalMode>, out_seq: *mut rosidl_runtime_rs::Sequence<OperationalMode>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__OperationalMode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OperationalMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: rosidl_runtime_rs::String,

}

impl OperationalMode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const MANUAL: &'static str = "MANUAL";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AUTOMATIC: &'static str = "AUTOMATIC";

}


impl Default for OperationalMode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__OperationalMode__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__OperationalMode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OperationalMode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__OperationalMode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__OperationalMode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__OperationalMode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OperationalMode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OperationalMode where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/OperationalMode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__OperationalMode() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__UserRole() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__UserRole__init(msg: *mut UserRole) -> bool;
    fn ur_dashboard_msgs__msg__UserRole__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UserRole>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__UserRole__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UserRole>);
    fn ur_dashboard_msgs__msg__UserRole__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UserRole>, out_seq: *mut rosidl_runtime_rs::Sequence<UserRole>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__UserRole
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UserRole {

    // This member is not documented.
    #[allow(missing_docs)]
    pub role: rosidl_runtime_rs::String,

}

impl UserRole {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROGRAMMER: &'static str = "PROGRAMMER";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OPERATOR: &'static str = "OPERATOR";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NONE: &'static str = "NONE";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LOCKED: &'static str = "LOCKED";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESTRICTED: &'static str = "RESTRICTED";

}


impl Default for UserRole {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__UserRole__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__UserRole__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UserRole {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__UserRole__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__UserRole__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__UserRole__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UserRole {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UserRole where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/UserRole";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__UserRole() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__VersionInformation() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__VersionInformation__init(msg: *mut VersionInformation) -> bool;
    fn ur_dashboard_msgs__msg__VersionInformation__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VersionInformation>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__VersionInformation__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VersionInformation>);
    fn ur_dashboard_msgs__msg__VersionInformation__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VersionInformation>, out_seq: *mut rosidl_runtime_rs::Sequence<VersionInformation>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__VersionInformation
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VersionInformation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub major: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub minor: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bugfix: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub build: u32,

}



impl Default for VersionInformation {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__VersionInformation__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__VersionInformation__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VersionInformation {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__VersionInformation__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__VersionInformation__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__VersionInformation__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VersionInformation {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VersionInformation where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/VersionInformation";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__VersionInformation() }
  }
}


#[link(name = "ur_dashboard_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__SafetyStatus() -> *const std::ffi::c_void;
}

#[link(name = "ur_dashboard_msgs__rosidl_generator_c")]
extern "C" {
    fn ur_dashboard_msgs__msg__SafetyStatus__init(msg: *mut SafetyStatus) -> bool;
    fn ur_dashboard_msgs__msg__SafetyStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SafetyStatus>, size: usize) -> bool;
    fn ur_dashboard_msgs__msg__SafetyStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SafetyStatus>);
    fn ur_dashboard_msgs__msg__SafetyStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SafetyStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<SafetyStatus>) -> bool;
}

// Corresponds to ur_dashboard_msgs__msg__SafetyStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafetyStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,

}

impl SafetyStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NORMAL: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const REDUCED: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROTECTIVE_STOP: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RECOVERY: i8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SAFEGUARD_STOP: i8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SYSTEM_EMERGENCY_STOP: i8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROBOT_EMERGENCY_STOP: i8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const VIOLATION: i8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FAULT: i8 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const VALIDATE_JOINT_ID: i8 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNDEFINED_SAFETY_MODE: i8 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const AUTOMATIC_MODE_SAFEGUARD_STOP: i8 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SYSTEM_THREE_POSITION_ENABLING_STOP: i8 = 13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TP_THREE_POSITION_ENABLING_STOP: i8 = 14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IMMI_EMERGENCY_STOP: i8 = 15;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IMMI_SAFEGUARD_STOP: i8 = 16;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROFISAFE_WAITING_FOR_PARAMETERS: i8 = 17;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROFISAFE_AUTOMATIC_MODE_SAFEGUARD_STOP: i8 = 18;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROFISAFE_SAFEGUARD_STOP: i8 = 19;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PROFISAFE_EMERGENCY_STOP: i8 = 20;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SAFETY_API_SAFEGUARD_STOP: i8 = 22;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IO_PLANE_STOP: i8 = 23;

}


impl Default for SafetyStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ur_dashboard_msgs__msg__SafetyStatus__init(&mut msg as *mut _) {
        panic!("Call to ur_dashboard_msgs__msg__SafetyStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SafetyStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ur_dashboard_msgs__msg__SafetyStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SafetyStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SafetyStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ur_dashboard_msgs/msg/SafetyStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ur_dashboard_msgs__msg__SafetyStatus() }
  }
}



#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ur_dashboard_msgs__msg__ProgramInformation

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProgramInformation {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: std::string::String,


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
    pub program_state: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ProgramInformation::default())
  }
}

impl rosidl_runtime_rs::Message for ProgramInformation {
  type RmwMsg = super::msg::rmw::ProgramInformation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        description: msg.description.as_str().into(),
        created_date: msg.created_date,
        last_saved_date: msg.last_saved_date,
        last_modified_date: msg.last_modified_date,
        program_state: msg.program_state.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        description: msg.description.as_str().into(),
      created_date: msg.created_date,
      last_saved_date: msg.last_saved_date,
      last_modified_date: msg.last_modified_date,
        program_state: msg.program_state.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      description: msg.description.to_string(),
      created_date: msg.created_date,
      last_saved_date: msg.last_saved_date,
      last_modified_date: msg.last_modified_date,
      program_state: msg.program_state.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__ProgramState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProgramState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ProgramState::default())
  }
}

impl rosidl_runtime_rs::Message for ProgramState {
  type RmwMsg = super::msg::rmw::ProgramState;

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


// Corresponds to ur_dashboard_msgs__msg__RobotMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RobotMode::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode {
  type RmwMsg = super::msg::rmw::RobotMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__SafetyMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SafetyMode::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyMode {
  type RmwMsg = super::msg::rmw::SafetyMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__OperationalMode

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OperationalMode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OperationalMode::default())
  }
}

impl rosidl_runtime_rs::Message for OperationalMode {
  type RmwMsg = super::msg::rmw::OperationalMode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__UserRole

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UserRole {

    // This member is not documented.
    #[allow(missing_docs)]
    pub role: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UserRole::default())
  }
}

impl rosidl_runtime_rs::Message for UserRole {
  type RmwMsg = super::msg::rmw::UserRole;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        role: msg.role.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        role: msg.role.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      role: msg.role.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__VersionInformation

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VersionInformation::default())
  }
}

impl rosidl_runtime_rs::Message for VersionInformation {
  type RmwMsg = super::msg::rmw::VersionInformation;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        major: msg.major,
        minor: msg.minor,
        bugfix: msg.bugfix,
        build: msg.build,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      major: msg.major,
      minor: msg.minor,
      bugfix: msg.bugfix,
      build: msg.build,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      major: msg.major,
      minor: msg.minor,
      bugfix: msg.bugfix,
      build: msg.build,
    }
  }
}


// Corresponds to ur_dashboard_msgs__msg__SafetyStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SafetyStatus::default())
  }
}

impl rosidl_runtime_rs::Message for SafetyStatus {
  type RmwMsg = super::msg::rmw::SafetyStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}



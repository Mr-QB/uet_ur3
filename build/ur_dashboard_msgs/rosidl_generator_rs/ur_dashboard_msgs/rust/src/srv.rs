#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to ur_dashboard_msgs__srv__AddToLog_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddToLog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for AddToLog_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddToLog_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddToLog_Request {
  type RmwMsg = super::srv::rmw::AddToLog_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__AddToLog_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddToLog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for AddToLog_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddToLog_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddToLog_Response {
  type RmwMsg = super::srv::rmw::AddToLog_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__DownloadProgram_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DownloadProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub target_path: std::string::String,

}



impl Default for DownloadProgram_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DownloadProgram_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DownloadProgram_Request {
  type RmwMsg = super::srv::rmw::DownloadProgram_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        program_name: msg.program_name.as_str().into(),
        target_path: msg.target_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        program_name: msg.program_name.as_str().into(),
        target_path: msg.target_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      program_name: msg.program_name.to_string(),
      target_path: msg.target_path.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__DownloadProgram_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DownloadProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for DownloadProgram_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DownloadProgram_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DownloadProgram_Response {
  type RmwMsg = super::srv::rmw::DownloadProgram_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetLoadedProgram_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLoadedProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetLoadedProgram_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLoadedProgram_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetLoadedProgram_Request {
  type RmwMsg = super::srv::rmw::GetLoadedProgram_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetLoadedProgram_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetLoadedProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetLoadedProgram_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetLoadedProgram_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetLoadedProgram_Response {
  type RmwMsg = super::srv::rmw::GetLoadedProgram_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      program_name: msg.program_name.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetPrograms_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPrograms_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPrograms_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPrograms_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPrograms_Request {
  type RmwMsg = super::srv::rmw::GetPrograms_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetPrograms_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPrograms_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub programs: Vec<super::msg::ProgramInformation>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetPrograms_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPrograms_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPrograms_Response {
  type RmwMsg = super::srv::rmw::GetPrograms_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        programs: msg.programs
          .into_iter()
          .map(|elem| super::msg::ProgramInformation::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        programs: msg.programs
          .iter()
          .map(|elem| super::msg::ProgramInformation::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      programs: msg.programs
          .into_iter()
          .map(super::msg::ProgramInformation::from_rmw_message)
          .collect(),
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetProgramState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetProgramState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetProgramState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetProgramState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetProgramState_Request {
  type RmwMsg = super::srv::rmw::GetProgramState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetProgramState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetProgramState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub state: super::msg::ProgramState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetProgramState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetProgramState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetProgramState_Response {
  type RmwMsg = super::srv::rmw::GetProgramState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::ProgramState::into_rmw_message(std::borrow::Cow::Owned(msg.state)).into_owned(),
        program_name: msg.program_name.as_str().into(),
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        state: super::msg::ProgramState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.state)).into_owned(),
        program_name: msg.program_name.as_str().into(),
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      state: super::msg::ProgramState::from_rmw_message(msg.state),
      program_name: msg.program_name.to_string(),
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetRobotMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetRobotMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotMode_Request {
  type RmwMsg = super::srv::rmw::GetRobotMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetRobotMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_mode: super::msg::RobotMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetRobotMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotMode_Response {
  type RmwMsg = super::srv::rmw::GetRobotMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_mode: super::msg::RobotMode::into_rmw_message(std::borrow::Cow::Owned(msg.robot_mode)).into_owned(),
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_mode: super::msg::RobotMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.robot_mode)).into_owned(),
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_mode: super::msg::RobotMode::from_rmw_message(msg.robot_mode),
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSafetyMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSafetyMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSafetyMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetSafetyMode_Request {
  type RmwMsg = super::srv::rmw::GetSafetyMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSafetyMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_mode: super::msg::SafetyMode,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetSafetyMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSafetyMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetSafetyMode_Response {
  type RmwMsg = super::srv::rmw::GetSafetyMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_mode: super::msg::SafetyMode::into_rmw_message(std::borrow::Cow::Owned(msg.safety_mode)).into_owned(),
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        safety_mode: super::msg::SafetyMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.safety_mode)).into_owned(),
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      safety_mode: super::msg::SafetyMode::from_rmw_message(msg.safety_mode),
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsProgramRunning_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramRunning_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsProgramRunning_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsProgramRunning_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IsProgramRunning_Request {
  type RmwMsg = super::srv::rmw::IsProgramRunning_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsProgramRunning_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramRunning_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

    /// is a program running?
    pub program_running: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsProgramRunning_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsProgramRunning_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IsProgramRunning_Response {
  type RmwMsg = super::srv::rmw::IsProgramRunning_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_running: msg.program_running,
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      program_running: msg.program_running,
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      program_running: msg.program_running,
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsProgramSaved_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramSaved_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsProgramSaved_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsProgramSaved_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IsProgramSaved_Request {
  type RmwMsg = super::srv::rmw::IsProgramSaved_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsProgramSaved_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsProgramSaved_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: std::string::String,

    /// is the current program saved?
    pub program_saved: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsProgramSaved_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsProgramSaved_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IsProgramSaved_Response {
  type RmwMsg = super::srv::rmw::IsProgramSaved_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
        program_saved: msg.program_saved,
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
      program_saved: msg.program_saved,
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      program_name: msg.program_name.to_string(),
      program_saved: msg.program_saved,
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__Load_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Load_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,

}



impl Default for Load_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Load_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Load_Request {
  type RmwMsg = super::srv::rmw::Load_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__Load_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Load_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for Load_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Load_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Load_Response {
  type RmwMsg = super::srv::rmw::Load_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__Popup_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Popup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Popup_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Popup_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Popup_Request {
  type RmwMsg = super::srv::rmw::Popup_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__Popup_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Popup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for Popup_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Popup_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Popup_Response {
  type RmwMsg = super::srv::rmw::Popup_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__RawRequest_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawRequest_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub query: std::string::String,

}



impl Default for RawRequest_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RawRequest_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RawRequest_Request {
  type RmwMsg = super::srv::rmw::RawRequest_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        query: msg.query.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        query: msg.query.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      query: msg.query.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__RawRequest_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RawRequest_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

}



impl Default for RawRequest_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RawRequest_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RawRequest_Response {
  type RmwMsg = super::srv::rmw::RawRequest_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsInRemoteControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsInRemoteControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for IsInRemoteControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsInRemoteControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for IsInRemoteControl_Request {
  type RmwMsg = super::srv::rmw::IsInRemoteControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__IsInRemoteControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IsInRemoteControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

    /// is the robot in remote control?
    pub remote_control: bool,

    /// Did the dashboard server call succeed?
    pub success: bool,

}



impl Default for IsInRemoteControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::IsInRemoteControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for IsInRemoteControl_Response {
  type RmwMsg = super::srv::rmw::IsInRemoteControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        remote_control: msg.remote_control,
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      remote_control: msg.remote_control,
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      remote_control: msg.remote_control,
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__UploadProgram_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadProgram_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub file_path: std::string::String,

}



impl Default for UploadProgram_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadProgram_Request::default())
  }
}

impl rosidl_runtime_rs::Message for UploadProgram_Request {
  type RmwMsg = super::srv::rmw::UploadProgram_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        file_path: msg.file_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      file_path: msg.file_path.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__UploadProgram_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UploadProgram_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub program_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for UploadProgram_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::UploadProgram_Response::default())
  }
}

impl rosidl_runtime_rs::Message for UploadProgram_Response {
  type RmwMsg = super::srv::rmw::UploadProgram_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        program_name: msg.program_name.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      program_name: msg.program_name.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetPolyScopeVersion_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPolyScopeVersion_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPolyScopeVersion_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPolyScopeVersion_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPolyScopeVersion_Request {
  type RmwMsg = super::srv::rmw::GetPolyScopeVersion_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetPolyScopeVersion_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPolyScopeVersion_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: super::msg::VersionInformation,

}



impl Default for GetPolyScopeVersion_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPolyScopeVersion_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPolyScopeVersion_Response {
  type RmwMsg = super::srv::rmw::GetPolyScopeVersion_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
        version: super::msg::VersionInformation::into_rmw_message(std::borrow::Cow::Owned(msg.version)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
        version: super::msg::VersionInformation::into_rmw_message(std::borrow::Cow::Borrowed(&msg.version)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
      version: super::msg::VersionInformation::from_rmw_message(msg.version),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSerialNumber_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSerialNumber_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSerialNumber_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSerialNumber_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetSerialNumber_Request {
  type RmwMsg = super::srv::rmw::GetSerialNumber_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSerialNumber_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSerialNumber_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub serial_number: u64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetSerialNumber_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSerialNumber_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetSerialNumber_Response {
  type RmwMsg = super::srv::rmw::GetSerialNumber_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        serial_number: msg.serial_number,
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      serial_number: msg.serial_number,
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      serial_number: msg.serial_number,
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetUserRole_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetUserRole_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetUserRole_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetUserRole_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetUserRole_Request {
  type RmwMsg = super::srv::rmw::GetUserRole_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetUserRole_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetUserRole_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user_role: super::msg::UserRole,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for GetUserRole_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetUserRole_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetUserRole_Response {
  type RmwMsg = super::srv::rmw::GetUserRole_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user_role: super::msg::UserRole::into_rmw_message(std::borrow::Cow::Owned(msg.user_role)).into_owned(),
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user_role: super::msg::UserRole::into_rmw_message(std::borrow::Cow::Borrowed(&msg.user_role)).into_owned(),
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      user_role: super::msg::UserRole::from_rmw_message(msg.user_role),
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__SetUserRole_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUserRole_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user_role: super::msg::UserRole,

}



impl Default for SetUserRole_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetUserRole_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetUserRole_Request {
  type RmwMsg = super::srv::rmw::SetUserRole_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user_role: super::msg::UserRole::into_rmw_message(std::borrow::Cow::Owned(msg.user_role)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user_role: super::msg::UserRole::into_rmw_message(std::borrow::Cow::Borrowed(&msg.user_role)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      user_role: super::msg::UserRole::from_rmw_message(msg.user_role),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__SetUserRole_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUserRole_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,

}



impl Default for SetUserRole_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetUserRole_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetUserRole_Response {
  type RmwMsg = super::srv::rmw::SetUserRole_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        answer: msg.answer.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        answer: msg.answer.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      answer: msg.answer.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__SetOperationalMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOperationalMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub operational_mode: super::msg::OperationalMode,

}



impl Default for SetOperationalMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOperationalMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOperationalMode_Request {
  type RmwMsg = super::srv::rmw::SetOperationalMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        operational_mode: super::msg::OperationalMode::into_rmw_message(std::borrow::Cow::Owned(msg.operational_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        operational_mode: super::msg::OperationalMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.operational_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      operational_mode: super::msg::OperationalMode::from_rmw_message(msg.operational_mode),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__SetOperationalMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOperationalMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for SetOperationalMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOperationalMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOperationalMode_Response {
  type RmwMsg = super::srv::rmw::SetOperationalMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetOperationalMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOperationalMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetOperationalMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOperationalMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOperationalMode_Request {
  type RmwMsg = super::srv::rmw::GetOperationalMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetOperationalMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOperationalMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operational_mode: super::msg::OperationalMode,

}



impl Default for GetOperationalMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOperationalMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOperationalMode_Response {
  type RmwMsg = super::srv::rmw::GetOperationalMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        answer: msg.answer.as_str().into(),
        operational_mode: super::msg::OperationalMode::into_rmw_message(std::borrow::Cow::Owned(msg.operational_mode)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        answer: msg.answer.as_str().into(),
        operational_mode: super::msg::OperationalMode::into_rmw_message(std::borrow::Cow::Borrowed(&msg.operational_mode)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      answer: msg.answer.to_string(),
      operational_mode: super::msg::OperationalMode::from_rmw_message(msg.operational_mode),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetRobotModel_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotModel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetRobotModel_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotModel_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotModel_Request {
  type RmwMsg = super::srv::rmw::GetRobotModel_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetRobotModel_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetRobotModel_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_model: std::string::String,

}



impl Default for GetRobotModel_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetRobotModel_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetRobotModel_Response {
  type RmwMsg = super::srv::rmw::GetRobotModel_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        answer: msg.answer.as_str().into(),
        robot_model: msg.robot_model.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        answer: msg.answer.as_str().into(),
        robot_model: msg.robot_model.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      answer: msg.answer.to_string(),
      robot_model: msg.robot_model.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSafetyStatus_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyStatus_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetSafetyStatus_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSafetyStatus_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetSafetyStatus_Request {
  type RmwMsg = super::srv::rmw::GetSafetyStatus_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GetSafetyStatus_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetSafetyStatus_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub safety_status: super::msg::SafetyStatus,

}



impl Default for GetSafetyStatus_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetSafetyStatus_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetSafetyStatus_Response {
  type RmwMsg = super::srv::rmw::GetSafetyStatus_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
        success: msg.success,
        safety_status: super::msg::SafetyStatus::into_rmw_message(std::borrow::Cow::Owned(msg.safety_status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        answer: msg.answer.as_str().into(),
      success: msg.success,
        safety_status: super::msg::SafetyStatus::into_rmw_message(std::borrow::Cow::Borrowed(&msg.safety_status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      answer: msg.answer.to_string(),
      success: msg.success,
      safety_status: super::msg::SafetyStatus::from_rmw_message(msg.safety_status),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GenerateFlightReport_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateFlightReport_Request {
    /// Same default as in the dashboard server
    pub report_type: std::string::String,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GenerateFlightReport_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GenerateFlightReport_Request {
  type RmwMsg = super::srv::rmw::GenerateFlightReport_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        report_type: msg.report_type.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        report_type: msg.report_type.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      report_type: msg.report_type.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GenerateFlightReport_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateFlightReport_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub report_id: std::string::String,

}



impl Default for GenerateFlightReport_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GenerateFlightReport_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GenerateFlightReport_Response {
  type RmwMsg = super::srv::rmw::GenerateFlightReport_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        answer: msg.answer.as_str().into(),
        report_id: msg.report_id.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        answer: msg.answer.as_str().into(),
        report_id: msg.report_id.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      answer: msg.answer.to_string(),
      report_id: msg.report_id.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GenerateSupportFile_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateSupportFile_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub dir_path: std::string::String,

}



impl Default for GenerateSupportFile_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GenerateSupportFile_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GenerateSupportFile_Request {
  type RmwMsg = super::srv::rmw::GenerateSupportFile_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dir_path: msg.dir_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        dir_path: msg.dir_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      dir_path: msg.dir_path.to_string(),
    }
  }
}


// Corresponds to ur_dashboard_msgs__srv__GenerateSupportFile_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GenerateSupportFile_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub answer: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub generated_file_name: std::string::String,

}



impl Default for GenerateSupportFile_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GenerateSupportFile_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GenerateSupportFile_Response {
  type RmwMsg = super::srv::rmw::GenerateSupportFile_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        answer: msg.answer.as_str().into(),
        generated_file_name: msg.generated_file_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        answer: msg.answer.as_str().into(),
        generated_file_name: msg.generated_file_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      answer: msg.answer.to_string(),
      generated_file_name: msg.generated_file_name.to_string(),
    }
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



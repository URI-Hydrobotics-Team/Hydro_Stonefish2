#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to stonefish_ros2__srv__SonarSettings_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub range_min: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub range_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gain: f64,

}



impl Default for SonarSettings_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SonarSettings_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SonarSettings_Request {
  type RmwMsg = super::srv::rmw::SonarSettings_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        range_min: msg.range_min,
        range_max: msg.range_max,
        gain: msg.gain,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      range_min: msg.range_min,
      range_max: msg.range_max,
      gain: msg.gain,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      range_min: msg.range_min,
      range_max: msg.range_max,
      gain: msg.gain,
    }
  }
}


// Corresponds to stonefish_ros2__srv__SonarSettings_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SonarSettings_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SonarSettings_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SonarSettings_Response {
  type RmwMsg = super::srv::rmw::SonarSettings_Response;

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


// Corresponds to stonefish_ros2__srv__SonarSettings2_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings2_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub range_min: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub range_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rotation_min: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rotation_max: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gain: f64,

}



impl Default for SonarSettings2_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SonarSettings2_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SonarSettings2_Request {
  type RmwMsg = super::srv::rmw::SonarSettings2_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        range_min: msg.range_min,
        range_max: msg.range_max,
        rotation_min: msg.rotation_min,
        rotation_max: msg.rotation_max,
        gain: msg.gain,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      range_min: msg.range_min,
      range_max: msg.range_max,
      rotation_min: msg.rotation_min,
      rotation_max: msg.rotation_max,
      gain: msg.gain,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      range_min: msg.range_min,
      range_max: msg.range_max,
      rotation_min: msg.rotation_min,
      rotation_max: msg.rotation_max,
      gain: msg.gain,
    }
  }
}


// Corresponds to stonefish_ros2__srv__SonarSettings2_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings2_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SonarSettings2_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SonarSettings2_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SonarSettings2_Response {
  type RmwMsg = super::srv::rmw::SonarSettings2_Response;

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


// Corresponds to stonefish_ros2__srv__Respawn_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Respawn_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub origin: geometry_msgs::msg::Pose,

}



impl Default for Respawn_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Respawn_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Respawn_Request {
  type RmwMsg = super::srv::rmw::Respawn_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        origin: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.origin)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        origin: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.origin)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      origin: geometry_msgs::msg::Pose::from_rmw_message(msg.origin),
    }
  }
}


// Corresponds to stonefish_ros2__srv__Respawn_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Respawn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Respawn_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Respawn_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Respawn_Response {
  type RmwMsg = super::srv::rmw::Respawn_Response;

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






#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__SonarSettings() -> *const std::ffi::c_void;
}

// Corresponds to stonefish_ros2__srv__SonarSettings
#[allow(missing_docs, non_camel_case_types)]
pub struct SonarSettings;

impl rosidl_runtime_rs::Service for SonarSettings {
    type Request = SonarSettings_Request;
    type Response = SonarSettings_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__SonarSettings() }
    }
}




#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__SonarSettings2() -> *const std::ffi::c_void;
}

// Corresponds to stonefish_ros2__srv__SonarSettings2
#[allow(missing_docs, non_camel_case_types)]
pub struct SonarSettings2;

impl rosidl_runtime_rs::Service for SonarSettings2 {
    type Request = SonarSettings2_Request;
    type Response = SonarSettings2_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__SonarSettings2() }
    }
}




#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__Respawn() -> *const std::ffi::c_void;
}

// Corresponds to stonefish_ros2__srv__Respawn
#[allow(missing_docs, non_camel_case_types)]
pub struct Respawn;

impl rosidl_runtime_rs::Service for Respawn {
    type Request = Respawn_Request;
    type Response = Respawn_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__stonefish_ros2__srv__Respawn() }
    }
}



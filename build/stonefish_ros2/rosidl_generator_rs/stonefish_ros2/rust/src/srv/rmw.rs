#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings_Request() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__SonarSettings_Request__init(msg: *mut SonarSettings_Request) -> bool;
    fn stonefish_ros2__srv__SonarSettings_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Request>, size: usize) -> bool;
    fn stonefish_ros2__srv__SonarSettings_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Request>);
    fn stonefish_ros2__srv__SonarSettings_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SonarSettings_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Request>) -> bool;
}

// Corresponds to stonefish_ros2__srv__SonarSettings_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__SonarSettings_Request__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__SonarSettings_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SonarSettings_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SonarSettings_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SonarSettings_Request where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/SonarSettings_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings_Request() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings_Response() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__SonarSettings_Response__init(msg: *mut SonarSettings_Response) -> bool;
    fn stonefish_ros2__srv__SonarSettings_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Response>, size: usize) -> bool;
    fn stonefish_ros2__srv__SonarSettings_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Response>);
    fn stonefish_ros2__srv__SonarSettings_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SonarSettings_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SonarSettings_Response>) -> bool;
}

// Corresponds to stonefish_ros2__srv__SonarSettings_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SonarSettings_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__SonarSettings_Response__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__SonarSettings_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SonarSettings_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SonarSettings_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SonarSettings_Response where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/SonarSettings_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings_Response() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings2_Request() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__SonarSettings2_Request__init(msg: *mut SonarSettings2_Request) -> bool;
    fn stonefish_ros2__srv__SonarSettings2_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Request>, size: usize) -> bool;
    fn stonefish_ros2__srv__SonarSettings2_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Request>);
    fn stonefish_ros2__srv__SonarSettings2_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SonarSettings2_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Request>) -> bool;
}

// Corresponds to stonefish_ros2__srv__SonarSettings2_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__SonarSettings2_Request__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__SonarSettings2_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SonarSettings2_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SonarSettings2_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SonarSettings2_Request where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/SonarSettings2_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings2_Request() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings2_Response() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__SonarSettings2_Response__init(msg: *mut SonarSettings2_Response) -> bool;
    fn stonefish_ros2__srv__SonarSettings2_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Response>, size: usize) -> bool;
    fn stonefish_ros2__srv__SonarSettings2_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Response>);
    fn stonefish_ros2__srv__SonarSettings2_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SonarSettings2_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SonarSettings2_Response>) -> bool;
}

// Corresponds to stonefish_ros2__srv__SonarSettings2_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SonarSettings2_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SonarSettings2_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__SonarSettings2_Response__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__SonarSettings2_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SonarSettings2_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__SonarSettings2_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SonarSettings2_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SonarSettings2_Response where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/SonarSettings2_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__SonarSettings2_Response() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__Respawn_Request() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__Respawn_Request__init(msg: *mut Respawn_Request) -> bool;
    fn stonefish_ros2__srv__Respawn_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Respawn_Request>, size: usize) -> bool;
    fn stonefish_ros2__srv__Respawn_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Respawn_Request>);
    fn stonefish_ros2__srv__Respawn_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Respawn_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Respawn_Request>) -> bool;
}

// Corresponds to stonefish_ros2__srv__Respawn_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Respawn_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub origin: geometry_msgs::msg::rmw::Pose,

}



impl Default for Respawn_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__Respawn_Request__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__Respawn_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Respawn_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Respawn_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Respawn_Request where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/Respawn_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__Respawn_Request() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__Respawn_Response() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__srv__Respawn_Response__init(msg: *mut Respawn_Response) -> bool;
    fn stonefish_ros2__srv__Respawn_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Respawn_Response>, size: usize) -> bool;
    fn stonefish_ros2__srv__Respawn_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Respawn_Response>);
    fn stonefish_ros2__srv__Respawn_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Respawn_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Respawn_Response>) -> bool;
}

// Corresponds to stonefish_ros2__srv__Respawn_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Respawn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Respawn_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__srv__Respawn_Response__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__srv__Respawn_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Respawn_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__srv__Respawn_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Respawn_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Respawn_Response where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/srv/Respawn_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__srv__Respawn_Response() }
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



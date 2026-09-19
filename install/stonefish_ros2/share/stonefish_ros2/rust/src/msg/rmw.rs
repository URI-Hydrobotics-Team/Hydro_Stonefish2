#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__Int32Stamped() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__Int32Stamped__init(msg: *mut Int32Stamped) -> bool;
    fn stonefish_ros2__msg__Int32Stamped__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Int32Stamped>, size: usize) -> bool;
    fn stonefish_ros2__msg__Int32Stamped__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Int32Stamped>);
    fn stonefish_ros2__msg__Int32Stamped__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Int32Stamped>, out_seq: *mut rosidl_runtime_rs::Sequence<Int32Stamped>) -> bool;
}

// Corresponds to stonefish_ros2__msg__Int32Stamped
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Int32Stamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: i32,

}



impl Default for Int32Stamped {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__Int32Stamped__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__Int32Stamped__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Int32Stamped {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Int32Stamped__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Int32Stamped__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Int32Stamped__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Int32Stamped {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Int32Stamped where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/Int32Stamped";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__Int32Stamped() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__BeaconInfo() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__BeaconInfo__init(msg: *mut BeaconInfo) -> bool;
    fn stonefish_ros2__msg__BeaconInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BeaconInfo>, size: usize) -> bool;
    fn stonefish_ros2__msg__BeaconInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BeaconInfo>);
    fn stonefish_ros2__msg__BeaconInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BeaconInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<BeaconInfo>) -> bool;
}

// Corresponds to stonefish_ros2__msg__BeaconInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BeaconInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub beacon_id: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub range: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub azimuth: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub elevation: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub relative_position: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub local_orientation: geometry_msgs::msg::rmw::Quaternion,


    // This member is not documented.
    #[allow(missing_docs)]
    pub local_depth: f32,

}



impl Default for BeaconInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__BeaconInfo__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__BeaconInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BeaconInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__BeaconInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__BeaconInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__BeaconInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BeaconInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BeaconInfo where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/BeaconInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__BeaconInfo() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__NEDPose() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__NEDPose__init(msg: *mut NEDPose) -> bool;
    fn stonefish_ros2__msg__NEDPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NEDPose>, size: usize) -> bool;
    fn stonefish_ros2__msg__NEDPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NEDPose>);
    fn stonefish_ros2__msg__NEDPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NEDPose>, out_seq: *mut rosidl_runtime_rs::Sequence<NEDPose>) -> bool;
}

// Corresponds to stonefish_ros2__msg__NEDPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NEDPose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub north: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub east: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub down: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

}



impl Default for NEDPose {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__NEDPose__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__NEDPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NEDPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__NEDPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__NEDPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__NEDPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NEDPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NEDPose where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/NEDPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__NEDPose() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__INS() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__INS__init(msg: *mut INS) -> bool;
    fn stonefish_ros2__msg__INS__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<INS>, size: usize) -> bool;
    fn stonefish_ros2__msg__INS__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<INS>);
    fn stonefish_ros2__msg__INS__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<INS>, out_seq: *mut rosidl_runtime_rs::Sequence<INS>) -> bool;
}

// Corresponds to stonefish_ros2__msg__INS
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct INS {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Global position of robot
    pub latitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub longitude: f64,

    /// Global position of NED origin
    pub origin_latitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub origin_longitude: f64,

    /// Robot pose in NED
    pub pose: super::super::msg::rmw::NEDPose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_variance: super::super::msg::rmw::NEDPose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub altitude: f64,

    /// Robot body velocity
    pub body_velocity: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpy_rate: geometry_msgs::msg::rmw::Vector3,

}



impl Default for INS {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__INS__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__INS__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for INS {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__INS__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__INS__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__INS__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for INS {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for INS where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/INS";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__INS() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DVLBeam() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__DVLBeam__init(msg: *mut DVLBeam) -> bool;
    fn stonefish_ros2__msg__DVLBeam__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DVLBeam>, size: usize) -> bool;
    fn stonefish_ros2__msg__DVLBeam__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DVLBeam>);
    fn stonefish_ros2__msg__DVLBeam__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DVLBeam>, out_seq: *mut rosidl_runtime_rs::Sequence<DVLBeam>) -> bool;
}

// Corresponds to stonefish_ros2__msg__DVLBeam
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Extracted from UUV Simulator:
///   https://github.com/uuvsimulator/uuv_simulator

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DVLBeam {
    /// measured range or < 0 if invalid
    pub range: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub range_covariance: f64,

    /// measured velocity of corr. beam
    pub velocity: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity_covariance: f64,

    /// Beam link pose wrt DVL link frame
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for DVLBeam {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__DVLBeam__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__DVLBeam__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DVLBeam {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVLBeam__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVLBeam__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVLBeam__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DVLBeam {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DVLBeam where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/DVLBeam";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DVLBeam() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DVL() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__DVL__init(msg: *mut DVL) -> bool;
    fn stonefish_ros2__msg__DVL__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DVL>, size: usize) -> bool;
    fn stonefish_ros2__msg__DVL__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DVL>);
    fn stonefish_ros2__msg__DVL__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DVL>, out_seq: *mut rosidl_runtime_rs::Sequence<DVL>) -> bool;
}

// Corresponds to stonefish_ros2__msg__DVL
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Extracted from UUV Simulator:
///   https://github.com/uuvsimulator/uuv_simulator

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DVL {
    /// This is a message to hold data from a DVL sensor (Doppler Velocity Log).
    ///
    /// Distances are in [m], velocities in [m/s]
    ///
    /// If the covariance is known, it should be filled.
    /// If it is unknown, it should be set to all zeros.
    /// If a measurement was invalid, its covariance should be set to -1 so it can be
    /// disregarded.
    ///
    /// DVLBeams are optional. If they are set they contain individual ranges and 1D
    /// doppler velocity estimates orthogonal to the ray.
    pub header: std_msgs::msg::rmw::Header,

    /// Measured velocity
    pub velocity: geometry_msgs::msg::rmw::Vector3,

    /// Row major, xyz axes
    pub velocity_covariance: [f64; 9],

    /// Altitude of the vehicle
    pub altitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub beams: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DVLBeam>,

}



impl Default for DVL {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__DVL__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__DVL__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DVL {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVL__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVL__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DVL__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DVL {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DVL where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/DVL";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DVL() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__ThrusterState() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__ThrusterState__init(msg: *mut ThrusterState) -> bool;
    fn stonefish_ros2__msg__ThrusterState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ThrusterState>, size: usize) -> bool;
    fn stonefish_ros2__msg__ThrusterState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ThrusterState>);
    fn stonefish_ros2__msg__ThrusterState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ThrusterState>, out_seq: *mut rosidl_runtime_rs::Sequence<ThrusterState>) -> bool;
}

// Corresponds to stonefish_ros2__msg__ThrusterState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ThrusterState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub setpoint: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpm: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub thrust: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for ThrusterState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__ThrusterState__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__ThrusterState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ThrusterState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__ThrusterState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__ThrusterState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__ThrusterState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ThrusterState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ThrusterState where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/ThrusterState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__ThrusterState() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DebugPhysics() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__DebugPhysics__init(msg: *mut DebugPhysics) -> bool;
    fn stonefish_ros2__msg__DebugPhysics__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DebugPhysics>, size: usize) -> bool;
    fn stonefish_ros2__msg__DebugPhysics__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DebugPhysics>);
    fn stonefish_ros2__msg__DebugPhysics__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DebugPhysics>, out_seq: *mut rosidl_runtime_rs::Sequence<DebugPhysics>) -> bool;
}

// Corresponds to stonefish_ros2__msg__DebugPhysics
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugPhysics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mass: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub volume: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub surface: f64,

    /// In local frame (origin)
    pub cog: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cob: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub inertia: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_coeff: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub skin_friction_coeff: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::rmw::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub buoyancy: geometry_msgs::msg::rmw::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping: geometry_msgs::msg::rmw::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub skin_friction: geometry_msgs::msg::rmw::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wetted_surface: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submerged_volume: f64,

}



impl Default for DebugPhysics {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__DebugPhysics__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__DebugPhysics__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DebugPhysics {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DebugPhysics__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DebugPhysics__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__DebugPhysics__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DebugPhysics {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DebugPhysics where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/DebugPhysics";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__DebugPhysics() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__Event() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__Event__init(msg: *mut Event) -> bool;
    fn stonefish_ros2__msg__Event__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Event>, size: usize) -> bool;
    fn stonefish_ros2__msg__Event__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Event>);
    fn stonefish_ros2__msg__Event__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Event>, out_seq: *mut rosidl_runtime_rs::Sequence<Event>) -> bool;
}

// Corresponds to stonefish_ros2__msg__Event
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Event {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ts: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polarity: bool,

}



impl Default for Event {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__Event__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__Event__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Event {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Event__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Event__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__Event__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Event {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Event where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/Event";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__Event() }
  }
}


#[link(name = "stonefish_ros2__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__EventArray() -> *const std::ffi::c_void;
}

#[link(name = "stonefish_ros2__rosidl_generator_c")]
extern "C" {
    fn stonefish_ros2__msg__EventArray__init(msg: *mut EventArray) -> bool;
    fn stonefish_ros2__msg__EventArray__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EventArray>, size: usize) -> bool;
    fn stonefish_ros2__msg__EventArray__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EventArray>);
    fn stonefish_ros2__msg__EventArray__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EventArray>, out_seq: *mut rosidl_runtime_rs::Sequence<EventArray>) -> bool;
}

// Corresponds to stonefish_ros2__msg__EventArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EventArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub height: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub width: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub events: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Event>,

}



impl Default for EventArray {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !stonefish_ros2__msg__EventArray__init(&mut msg as *mut _) {
        panic!("Call to stonefish_ros2__msg__EventArray__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EventArray {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__EventArray__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__EventArray__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { stonefish_ros2__msg__EventArray__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EventArray {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EventArray where Self: Sized {
  const TYPE_NAME: &'static str = "stonefish_ros2/msg/EventArray";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__stonefish_ros2__msg__EventArray() }
  }
}



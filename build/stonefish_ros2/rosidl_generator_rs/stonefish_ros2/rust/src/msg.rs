#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to stonefish_ros2__msg__Int32Stamped

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Int32Stamped {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: i32,

}



impl Default for Int32Stamped {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Int32Stamped::default())
  }
}

impl rosidl_runtime_rs::Message for Int32Stamped {
  type RmwMsg = super::msg::rmw::Int32Stamped;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      data: msg.data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      data: msg.data,
    }
  }
}


// Corresponds to stonefish_ros2__msg__BeaconInfo

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BeaconInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub relative_position: geometry_msgs::msg::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub local_orientation: geometry_msgs::msg::Quaternion,


    // This member is not documented.
    #[allow(missing_docs)]
    pub local_depth: f32,

}



impl Default for BeaconInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BeaconInfo::default())
  }
}

impl rosidl_runtime_rs::Message for BeaconInfo {
  type RmwMsg = super::msg::rmw::BeaconInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        beacon_id: msg.beacon_id,
        range: msg.range,
        azimuth: msg.azimuth,
        elevation: msg.elevation,
        relative_position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.relative_position)).into_owned(),
        local_orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.local_orientation)).into_owned(),
        local_depth: msg.local_depth,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      beacon_id: msg.beacon_id,
      range: msg.range,
      azimuth: msg.azimuth,
      elevation: msg.elevation,
        relative_position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.relative_position)).into_owned(),
        local_orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.local_orientation)).into_owned(),
      local_depth: msg.local_depth,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      beacon_id: msg.beacon_id,
      range: msg.range,
      azimuth: msg.azimuth,
      elevation: msg.elevation,
      relative_position: geometry_msgs::msg::Point::from_rmw_message(msg.relative_position),
      local_orientation: geometry_msgs::msg::Quaternion::from_rmw_message(msg.local_orientation),
      local_depth: msg.local_depth,
    }
  }
}


// Corresponds to stonefish_ros2__msg__NEDPose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::NEDPose::default())
  }
}

impl rosidl_runtime_rs::Message for NEDPose {
  type RmwMsg = super::msg::rmw::NEDPose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        north: msg.north,
        east: msg.east,
        down: msg.down,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      north: msg.north,
      east: msg.east,
      down: msg.down,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      north: msg.north,
      east: msg.east,
      down: msg.down,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
    }
  }
}


// Corresponds to stonefish_ros2__msg__INS

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct INS {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    pub pose: super::msg::NEDPose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose_variance: super::msg::NEDPose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub altitude: f64,

    /// Robot body velocity
    pub body_velocity: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpy_rate: geometry_msgs::msg::Vector3,

}



impl Default for INS {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::INS::default())
  }
}

impl rosidl_runtime_rs::Message for INS {
  type RmwMsg = super::msg::rmw::INS;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        latitude: msg.latitude,
        longitude: msg.longitude,
        origin_latitude: msg.origin_latitude,
        origin_longitude: msg.origin_longitude,
        pose: super::msg::NEDPose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        pose_variance: super::msg::NEDPose::into_rmw_message(std::borrow::Cow::Owned(msg.pose_variance)).into_owned(),
        altitude: msg.altitude,
        body_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.body_velocity)).into_owned(),
        rpy_rate: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.rpy_rate)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      latitude: msg.latitude,
      longitude: msg.longitude,
      origin_latitude: msg.origin_latitude,
      origin_longitude: msg.origin_longitude,
        pose: super::msg::NEDPose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        pose_variance: super::msg::NEDPose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose_variance)).into_owned(),
      altitude: msg.altitude,
        body_velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.body_velocity)).into_owned(),
        rpy_rate: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.rpy_rate)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      latitude: msg.latitude,
      longitude: msg.longitude,
      origin_latitude: msg.origin_latitude,
      origin_longitude: msg.origin_longitude,
      pose: super::msg::NEDPose::from_rmw_message(msg.pose),
      pose_variance: super::msg::NEDPose::from_rmw_message(msg.pose_variance),
      altitude: msg.altitude,
      body_velocity: geometry_msgs::msg::Vector3::from_rmw_message(msg.body_velocity),
      rpy_rate: geometry_msgs::msg::Vector3::from_rmw_message(msg.rpy_rate),
    }
  }
}


// Corresponds to stonefish_ros2__msg__DVLBeam
/// Extracted from UUV Simulator:
///   https://github.com/uuvsimulator/uuv_simulator

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub pose: geometry_msgs::msg::PoseStamped,

}



impl Default for DVLBeam {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DVLBeam::default())
  }
}

impl rosidl_runtime_rs::Message for DVLBeam {
  type RmwMsg = super::msg::rmw::DVLBeam;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        range: msg.range,
        range_covariance: msg.range_covariance,
        velocity: msg.velocity,
        velocity_covariance: msg.velocity_covariance,
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      range: msg.range,
      range_covariance: msg.range_covariance,
      velocity: msg.velocity,
      velocity_covariance: msg.velocity_covariance,
        pose: geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      range: msg.range,
      range_covariance: msg.range_covariance,
      velocity: msg.velocity,
      velocity_covariance: msg.velocity_covariance,
      pose: geometry_msgs::msg::PoseStamped::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to stonefish_ros2__msg__DVL
/// Extracted from UUV Simulator:
///   https://github.com/uuvsimulator/uuv_simulator

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub header: std_msgs::msg::Header,

    /// Measured velocity
    pub velocity: geometry_msgs::msg::Vector3,

    /// Row major, xyz axes
    pub velocity_covariance: [f64; 9],

    /// Altitude of the vehicle
    pub altitude: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub beams: Vec<super::msg::DVLBeam>,

}



impl Default for DVL {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DVL::default())
  }
}

impl rosidl_runtime_rs::Message for DVL {
  type RmwMsg = super::msg::rmw::DVL;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        velocity_covariance: msg.velocity_covariance,
        altitude: msg.altitude,
        beams: msg.beams
          .into_iter()
          .map(|elem| super::msg::DVLBeam::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
        velocity_covariance: msg.velocity_covariance,
      altitude: msg.altitude,
        beams: msg.beams
          .iter()
          .map(|elem| super::msg::DVLBeam::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      velocity: geometry_msgs::msg::Vector3::from_rmw_message(msg.velocity),
      velocity_covariance: msg.velocity_covariance,
      altitude: msg.altitude,
      beams: msg.beams
          .into_iter()
          .map(super::msg::DVLBeam::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to stonefish_ros2__msg__ThrusterState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ThrusterState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub setpoint: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpm: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub thrust: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque: Vec<f64>,

}



impl Default for ThrusterState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ThrusterState::default())
  }
}

impl rosidl_runtime_rs::Message for ThrusterState {
  type RmwMsg = super::msg::rmw::ThrusterState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        setpoint: msg.setpoint.as_slice().into(),
        rpm: msg.rpm.as_slice().into(),
        thrust: msg.thrust.as_slice().into(),
        torque: msg.torque.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        setpoint: msg.setpoint.as_slice().into(),
        rpm: msg.rpm.as_slice().into(),
        thrust: msg.thrust.as_slice().into(),
        torque: msg.torque.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      setpoint: msg.setpoint.into(),
      rpm: msg.rpm.into(),
      thrust: msg.thrust.into(),
      torque: msg.torque.into(),
    }
  }
}


// Corresponds to stonefish_ros2__msg__DebugPhysics

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugPhysics {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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
    pub cog: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cob: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub inertia: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_coeff: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub skin_friction_coeff: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub buoyancy: geometry_msgs::msg::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping: geometry_msgs::msg::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub skin_friction: geometry_msgs::msg::Wrench,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wetted_surface: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submerged_volume: f64,

}



impl Default for DebugPhysics {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DebugPhysics::default())
  }
}

impl rosidl_runtime_rs::Message for DebugPhysics {
  type RmwMsg = super::msg::rmw::DebugPhysics;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        mass: msg.mass,
        volume: msg.volume,
        surface: msg.surface,
        cog: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.cog)).into_owned(),
        cob: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.cob)).into_owned(),
        inertia: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.inertia)).into_owned(),
        damping_coeff: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.damping_coeff)).into_owned(),
        skin_friction_coeff: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.skin_friction_coeff)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        buoyancy: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.buoyancy)).into_owned(),
        damping: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.damping)).into_owned(),
        skin_friction: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Owned(msg.skin_friction)).into_owned(),
        wetted_surface: msg.wetted_surface,
        submerged_volume: msg.submerged_volume,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      mass: msg.mass,
      volume: msg.volume,
      surface: msg.surface,
        cog: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.cog)).into_owned(),
        cob: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.cob)).into_owned(),
        inertia: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.inertia)).into_owned(),
        damping_coeff: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.damping_coeff)).into_owned(),
        skin_friction_coeff: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.skin_friction_coeff)).into_owned(),
        velocity: geometry_msgs::msg::Twist::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
        buoyancy: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.buoyancy)).into_owned(),
        damping: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.damping)).into_owned(),
        skin_friction: geometry_msgs::msg::Wrench::into_rmw_message(std::borrow::Cow::Borrowed(&msg.skin_friction)).into_owned(),
      wetted_surface: msg.wetted_surface,
      submerged_volume: msg.submerged_volume,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      mass: msg.mass,
      volume: msg.volume,
      surface: msg.surface,
      cog: geometry_msgs::msg::Vector3::from_rmw_message(msg.cog),
      cob: geometry_msgs::msg::Vector3::from_rmw_message(msg.cob),
      inertia: geometry_msgs::msg::Vector3::from_rmw_message(msg.inertia),
      damping_coeff: geometry_msgs::msg::Vector3::from_rmw_message(msg.damping_coeff),
      skin_friction_coeff: geometry_msgs::msg::Vector3::from_rmw_message(msg.skin_friction_coeff),
      velocity: geometry_msgs::msg::Twist::from_rmw_message(msg.velocity),
      buoyancy: geometry_msgs::msg::Wrench::from_rmw_message(msg.buoyancy),
      damping: geometry_msgs::msg::Wrench::from_rmw_message(msg.damping),
      skin_friction: geometry_msgs::msg::Wrench::from_rmw_message(msg.skin_friction),
      wetted_surface: msg.wetted_surface,
      submerged_volume: msg.submerged_volume,
    }
  }
}


// Corresponds to stonefish_ros2__msg__Event

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub ts: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub polarity: bool,

}



impl Default for Event {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Event::default())
  }
}

impl rosidl_runtime_rs::Message for Event {
  type RmwMsg = super::msg::rmw::Event;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        ts: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.ts)).into_owned(),
        polarity: msg.polarity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
        ts: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.ts)).into_owned(),
      polarity: msg.polarity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      ts: builtin_interfaces::msg::Time::from_rmw_message(msg.ts),
      polarity: msg.polarity,
    }
  }
}


// Corresponds to stonefish_ros2__msg__EventArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EventArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub height: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub width: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub events: Vec<super::msg::Event>,

}



impl Default for EventArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EventArray::default())
  }
}

impl rosidl_runtime_rs::Message for EventArray {
  type RmwMsg = super::msg::rmw::EventArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        height: msg.height,
        width: msg.width,
        events: msg.events
          .into_iter()
          .map(|elem| super::msg::Event::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      height: msg.height,
      width: msg.width,
        events: msg.events
          .iter()
          .map(|elem| super::msg::Event::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      height: msg.height,
      width: msg.width,
      events: msg.events
          .into_iter()
          .map(super::msg::Event::from_rmw_message)
          .collect(),
    }
  }
}



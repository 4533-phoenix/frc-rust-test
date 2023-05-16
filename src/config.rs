//! Robot config structures
//!
//! The robot's configuration values are stored in a TOML config file for easy
//! tweaking.
//!
//! ## Motor naming scheme
//!
//! ```text
//! f l d
//! | | \_ drive
//! | \_________ left
//! \_________________ front
//! ```
//!
//!  - **`f` / `b`**: front / back
//!  - **`l` / `r`**: left / right
//!  - **`d` / `s`**: drive / steer

#[derive(Clone, Copy)]
pub struct Config {
  pub looper_time: f32,
  pub pcm_id: u8,

  /// Left-to-right distance between wheel centers in meters
  pub track_width: f32,
  /// Front-to-back distance between wheel centers in meters
  pub wheel_base: f32,

  /// Wheel diameter in meters
  pub wheel_diameter: f32,

  pub drive_gear_ratio: f32,
  pub steer_gear_ratio: f32,

  pub motors: Motors,

  pub encoders: Encoders,
  pub absolute_encoders: AbsoluteEncoders,
}

pub struct Module {
  pub wheel_diameter_inches: f32,
  pub wheel_diameter_meters: f32,

  pub drive_enc_meters_per_rot: f64,
  pub drive_enc_radians_per_rot: f64,
  pub drive_enc_meters_per_sec: f64,
  pub drive_enc_radians_per_sec: f64,

  pub steer_kp: f32,
  pub steer_ki: f32,
  pub steer_kd: f32,
}

/// Motor IDs
#[derive(Clone, Copy)]
pub struct Motors {
  /// `Front Left` *Drive* Motor ID
  pub fld: u8,
  /// `Front Right` *Drive* Motor ID
  pub frd: u8,
  /// `Back Left` *Drive* Motor ID
  pub bld: u8,
  /// `Back Right` *Drive* Motor ID
  pub brd: u8,

  /// `Front Left` *Steer* Motor ID
  pub fls: u8,
  /// `Front Right` *Steer* Motor ID
  pub frs: u8,
  /// `Back Left` *Steer* Motor ID
  pub bls: u8,
  /// `Back Right` *Steer* Motor ID
  pub brs: u8,
}

#[derive(Clone, Copy)]
pub struct Encoders {
  /// `Front Left` *Drive* Encoder Reversed
  pub fld_rev: bool,
  /// `Front Right` *Drive* Encoder Reversed
  pub frd_rev: bool,
  /// `Back Left` *Drive* Encoder Reversed
  pub bld_rev: bool,
  /// `Back Right` *Drive* Encoder Reversed
  pub brd_rev: bool,

  /// `Front Left` *Steer* Encoder Reversed
  pub fls_rev: bool,
  /// `Front Right` *Steer* Encoder Reversed
  pub frs_rev: bool,
  /// `Back Left` *Steer* Encoder Reversed
  pub bls_rev: bool,
  /// `Back Right` *Steer* Encoder Reversed
  pub brs_rev: bool,
}

/// Absolute encoder settings
///
/// ## `rev`
///
/// Whether or not the absolute encoder is reversed
///
/// ## `offset`
///
/// Offsets for swerve modules
///
/// Should be equal to absolute encoder reading when wheel is facing straight
/// forward
#[derive(Clone, Copy)]
pub struct AbsoluteEncoders {
  /// `Front Left` *Steer* Absoulte Encoder ID
  pub fls: u8,
  /// `Front Right` *Steer* Absolute Encoder ID
  pub frs: u8,
  /// `Back Left` *Steer* Absolute Encoder ID
  pub bls: u8,
  /// `Back Right` *Steer* Absolute Encoder ID
  pub brs: u8,

  /// `Front Left` *Steer* Absolute Encoder Reversed
  pub fls_rev: bool,
  /// `Front Right` *Steer* Absolute Encoder Reversed
  pub frs_rev: bool,
  /// `Back Left` *Steer* Absolute Encoder Reversed
  pub bls_rev: bool,
  /// `Back Right` *Steer* Absolute Encoder Reversed
  pub brs_rev: bool,

  /// `Front Left` *Steer* Absolute Encoder Offset
  pub fls_offset: f32,
  /// `Front Right` *Steer* Absolute Encoder Offset
  pub frs_offset: f32,
  /// `Back Left` *Steer* Absolute Encoder Offset
  pub bls_offset: f32,
  /// `Back Right` *Steer* Absolute Encoder Offset
  pub brs_offset: f32,
}

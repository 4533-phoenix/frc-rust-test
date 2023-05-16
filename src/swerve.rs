#[derive(Clone, Copy)]
pub struct SwerveModule {
    /// Drive Motor ID
    ///
    /// The CAN ID of the drive motor
    pub drive_id: u8,
    /// Steer Motor ID
    ///
    /// The CAN ID of the steer motor
    pub steer_id: u8,

    /// Drive Motor Reversed
    ///
    /// Inversion status of the drive motor controller
    pub drive_rev: bool,
    /// Steer Motor Reversed
    ///
    /// Inversion status of the steer motor controller
    pub steer_rev: bool,

    /// Absolute Encoder ID
    ///
    /// The CAN ID of the absolute encoder
    pub abs_enc_id: u8,
    /// Absolute Encoder Offset
    ///
    /// The offset of the absolute encoder from zero
    pub abs_enc_offset: f32,
    /// Absolute Encoder Reversed
    ///
    /// Inversion status of the absolute encoder
    pub abs_enc_rev: bool,

    drive_enc: Encode
}
impl SwerveModule {
    pub fn drive_pos(&self) -> f64;
    pub fn steer_pos(&self) -> f64;

    pub fn drive_velocity(&self) -> f64;
    pub fn steer_velocity(&self) -> f64;

    pub fn abs_enc_pos(&self) -> f64;

    pub fn reset_encoders(&self) {
        self.drive_enc.set_pos(0.0);
        self.steer_enc.set_pos(self.abs_enc_pos());
    }
}


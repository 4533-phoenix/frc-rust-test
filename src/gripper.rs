use std::error::Error;

use wpilib::{dio::DigitalInput, pneumatics::Solenoid};

use crate::config::Config;

pub struct Gripper {
  cylinder: Solenoid,
  limit_switch: DigitalInput,
}
impl Gripper {
  pub fn new(cfg: Config) -> Result<Self, Box<dyn Error>> {
    let cylinder = Solenoid::new()?;
    let limit_switch = DigitalInput::new()?;

    Ok(Self {
      cylinder,
      limit_switch,
    })
  }

  pub fn grip(&self) -> Result<(), Box<dyn Error>> {
    self.cylinder.set(false)?;
    Ok(())
  }
  pub fn release(&self) -> Result<(), Box<dyn Error>> {
    self.cylinder.set(true)?;
    Ok(())
  }

  pub fn obj_in_gripper(&self) -> Result<bool, Box<dyn Error>> {
    Ok(self.limit_switch.get()?)
  }
}

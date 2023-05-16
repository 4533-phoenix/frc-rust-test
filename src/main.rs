extern crate wpilib;

use std::error::Error;

use wpilib::RobotBase;

mod config;
mod gripper;

fn main() -> Result<(), Box<dyn Error>> {
  let mut r = RobotBase::new()?;
  //
  Ok(())
}

//! Implement calculation of forces, speed and acceleration on a 3D-axis.
//!
//! Allowes calculation of object mouvement over time based on acceleration, speed and position as well as forces.
//! This crate is no_std and no_alloc!
#![no_std]
use core::fmt;
#[cfg(feature = "std")]
use std::vec::Vec;

use libm::{cos,sin,pow, sqrt};
// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;
/// Deliminate a Place in 3D axis for elements
#[derive(Clone, Copy, Debug)]
pub struct Places {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
/// Give name to the 3 axis
#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}
impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Axis::X => write!(f, "X"),
            Axis::Y => write!(f, "Y"),
            Axis::Z => write!(f, "Z"),
        }
    }
}
/// Earth gravity constant
pub const EARTH_GRAVITY: f64 = 9.80665;
/// Define an object with a particular position, speed, acceleration and weight
#[derive(Clone, Copy, Debug)]
pub struct Object {
    /// Position in space
    pub position: Places,
    /// Speed according to space point (vector)
    pub speed: Places,
    /// Acceleration according to space point (acceleration)
    pub acceleration: Places,
    /// Weight (consistent with force so normally kg and N)
    pub weight: f64,
}
impl Places {
    /// Create a place
    /// ```
    /// use forces::*;
    /// Places::new(0.0,0.0,0.0);
    /// ```
    pub fn new<T: Into<f64> + Copy>(x: T, y: T, z: T) -> Self {
        let x = x.into();
        let y = y.into();
        let z = z.into();
        Places { x, y, z }
    }
}
impl IntoIterator for Places {
    type IntoIter = core::array::IntoIter<(Axis, f64), 3>;
    type Item = (Axis, f64);
    fn into_iter(self) -> Self::IntoIter {
        [(Axis::X, self.x), (Axis::Y, self.y), (Axis::Z, self.z)].into_iter()
    }
}
impl Object {
    /// Create an object
    /// ```
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(40.0,40.0,0.0);
    /// let acceleration = Places::new(0.0,-EARTH_GRAVITY,0.0);
    /// Object::new(startpoint, speed, acceleration, 30.0);
    /// ```
    pub fn new(position: Places, speed: Places, acceleration: Places, weight: f64) -> Self {
        let weight = if !weight.is_finite() { weight } else { 1.0 };
        Object {
            position,
            speed,
            acceleration,
            weight,
        }
    }
    /// Calculation position of object after x time
    /// ```
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(40.0,40.0,0.0);
    /// let acceleration = Places::new(0.0,-EARTH_GRAVITY,0.0);
    /// let object = Object::new(startpoint, speed, acceleration, 30.0);
    /// let object_t4 = object.overtime(4.0);
    /// ```
    pub fn overtime(&self, time: f64) -> Places {
        let mut places = Places::new(0.0, 0.0, 0.0);
        places.x = self.acceleration.x * time * time * 0.5 + self.speed.x * time + self.position.x;
        places.y = self.acceleration.y * time * time * 0.5 + self.speed.y * time + self.position.y;
        places.z = self.acceleration.z * time * time * 0.5 + self.speed.z * time + self.position.z;
        places
    }
    /// Same but changes the inner object
    pub fn overtime_mut(&mut self, time: f64) {
        self.position.x =
            self.acceleration.x * time * time * 0.5 + self.speed.x * time + self.position.x;
        self.position.y =
            self.acceleration.y * time * time * 0.5 + self.speed.y * time + self.position.y;
        self.position.z =
            self.acceleration.z * time * time * 0.5 + self.speed.z * time + self.position.z;
    }
    /// When one coordinate hits zero
    /// 
    /// ```
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(40.0,40.0,0.0);
    /// let acceleration = Places::new(0.0,-EARTH_GRAVITY,0.0);
    /// let object = Object::new(startpoint, speed, acceleration, 30.0);
    /// let zero = object.hitzero();
    /// ```
    /// It can returns Nan or infinity depending on input.
    /// 
    /// ```should_panic
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(0.0,0.0,0.0);
    /// let acceleration = Places::new(0.0,0.0,0.0);
    /// let object = Object::new(startpoint, speed, acceleration, 30.0);
    /// assert!(!object.hitzero().x.is_nan() || !object.hitzero().y.is_nan() || !object.hitzero().z.is_nan());
    /// ```
    pub fn hitzero(&self) -> Places {
        let (a, b, c) = (0.5 * self.acceleration.x, self.speed.x, self.position.x);
        let delta1 = pow(b, 2.0) - (4.0 * a * c);
        let delta_x = if a == 0.0 {
            -c / b
        } else {
            if a.is_sign_negative() {
                -b - sqrt(delta1) / (2.0 * a)
            } else {
                -b + sqrt(delta1) / (2.0 * a)
            }
        };
        let (a, b, c) = (0.5 * self.acceleration.y, self.speed.y, self.position.y);
        let delta1 = pow(b, 2.0) - (4.0 * a * c);
        let delta_y = if a == 0.0 {
            -c / b
        } else {
            if a.is_sign_negative() {
                (-b - sqrt(delta1)) / (2.0 * a)
            } else {
                (-b + sqrt(delta1)) / (2.0 * a)
            }
        };
        let (a, b, c) = (0.5 * self.acceleration.z, self.speed.z, self.position.z);
        let delta1 = pow(b, 2.0) - (4.0 * a * c);
        let delta_z = if a == 0.0 {
            -c / b
        } else {
            if a.is_sign_negative() {
                (-b - sqrt(delta1)) / (2.0 * a)
            } else {
                (-b + sqrt(delta1)) / (2.0 * a)
            }
        };
        Places::new(delta_x, delta_y, delta_z)
    }
    /// Add a force for unlimited time (0s) or for a specific amount of time on a specific axis.
    /// ```
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(40.0,40.0,0.0);
    /// let acceleration = Places::new(0.0,-EARTH_GRAVITY,0.0);
    /// let mut object = Object::new(startpoint, speed, acceleration, 30.0);
    /// object.addforce(40.0,5.0,Axis::Y);
    /// assert_eq!(object.speed.y,240.0);
    /// ```
    pub fn addforce(&mut self, force: f64, time: f64, axis: Axis) {
        if !time.is_finite() {
            return;
        }
        let data_to_change = match time == 0.0 {
            true => match axis {
                Axis::X => &mut self.acceleration.x,
                Axis::Y => &mut self.acceleration.y,
                Axis::Z => &mut self.acceleration.z,
            },
            false => match axis {
                Axis::X => &mut self.speed.x,
                Axis::Y => &mut self.speed.y,
                Axis::Z => &mut self.speed.z,
            },
        };
        *data_to_change += force / self.weight * (if time == 0.0 { 1.0 } else { time });
    }
    /// Split a force into two different forces on different axis based on angle
    /// ```
    /// use forces::*;
    /// let startpoint = Places::new(0.0,0.0,0.0);
    /// let speed = Places::new(40.0,40.0,0.0);
    /// let acceleration = Places::new(0.0,-EARTH_GRAVITY,0.0);
    /// let mut object = Object::new(startpoint, speed, acceleration, 30.0);
    /// let (forcex, forcey) = object.transverseforce(20.0, 60.0);
    /// object.addforce(forcex,5.0,Axis::X);
    /// object.addforce(forcey,5.0,Axis::Y);
    /// assert_eq!(object.speed.x,138.61432315629253);
    /// assert_eq!(object.speed.y,56.5896132693415);
    /// ```
    pub fn transverseforce(&self, force: f64, degrees: f64) -> (f64, f64) {
        (force * cos(degrees/360.0),force * sin(degrees/360.0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "std")]
    use std::string::ToString;

    #[test]
    fn it_works() {
        let position = Places::new(0, 0, 0);
        let speed = Places::new(10, 20, 0);
        let accelerator = Places::new(0.0, -EARTH_GRAVITY, 0.0);
        //let accelerator = Places::new(0,-9,0);
        let mut object = Object::new(position, speed, accelerator, 5.0);
        #[cfg(feature = "std")]
        {
            std::eprintln!("Finish at {} s. Result {:#?}", 2, object.overtime(2.0));
            std::eprintln!("Finish at {} s. Result {:#?}", 4, object.overtime(4.0));
            std::eprintln!("Finish at {} s. Result {:#?}", 8, object.overtime(8.0));
            std::eprintln!("Finish at {:#?}", object.hitzero());
        }
        object.addforce(10.0, 5.0, Axis::Y);
        #[cfg(feature = "std")]
        {
            let maps: Vec<(Axis)> = object
                .hitzero()
                .into_iter()
                .filter_map(|x| if x.1 == 0.0 { Some(x.0) } else { None })
                .collect();
            std::eprintln!("Finish at {} s. Result {:#?}", 2, object.overtime(2.0));
            std::eprintln!("Finish at {} s. Result {:#?}", 4, object.overtime(4.0));
            std::eprintln!("Finish at {} s. Result {:#?}", 8, object.overtime(8.0));
            std::eprintln!("Finish at {}", maps.get(0).unwrap());
            assert_eq!("X", maps.get(0).unwrap().to_string());
        }
        assert_eq!(object.acceleration.y, -EARTH_GRAVITY);
    }
}

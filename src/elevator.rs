use std::cmp::min;
use std::time::Duration;

use crate::entity::Entity;
use crate::entity::EntityType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum DoorState {
    Closed,
    Closing,
    Open,
    Opening,
}

pub(crate) struct Elevator {
    // maximum weight in kg
    max_weight: f32,
    // maximum capacity
    max_people: i8,
    // maximum velocity in m/s
    max_velocity: f32,
    // acceleration in m/s²
    pub(crate) acceleration: f32,
    // time it takes to open/close the doors
    door_close_duration: Duration,

    // weight in kg
    pub(crate) weight: f32,
    // velocity in m/s
    pub(crate) velocity: f32,
    // altitude in m
    pub(crate) altitude: f32,
    // target altitude in m
    pub(crate) target_altitude: f32,
    // 1 for up, -1 for down
    pub(crate) direction: f32,
    // state of the doors
    pub(crate) door_state: DoorState,
}

impl Elevator {
    pub(crate) fn new() -> Elevator {
        Elevator {
            max_weight: 250.0,
            max_people: 4,
            max_velocity: 1.6,
            acceleration: 0.6,
            door_close_duration: Duration::from_millis(2500),

            // state
            weight: 0.0,
            velocity: 0.0,
            altitude: 0.0,
            target_altitude: 6.0, // 6m = 2nd floor
            direction: 1.0,
            door_state: DoorState::Open,
        }
    }
}

impl Entity for Elevator {
    fn entity_type() -> EntityType {
        EntityType::Elevator
    }

    fn simulate(&mut self, time_delta: Duration) {
        if self.altitude != self.target_altitude {
            // update door state
            if self.door_state != DoorState::Closed {
                self.door_state = match self.door_state {
                    DoorState::Open => DoorState::Closing,
                    DoorState::Closing => DoorState::Closed,
                    DoorState::Opening => DoorState::Open,
                    _ => self.door_state
                };
                return;
            }

            assert_eq!(self.door_state, DoorState::Closed);

            // update velocity
            if self.velocity < self.max_velocity {
                self.velocity = self.velocity + self.acceleration * time_delta.as_secs_f32();
                if self.velocity > self.max_velocity {
                    self.velocity = self.max_velocity
                }
            }

            // update altitude
            self.altitude = self.altitude + self.direction * self.velocity * time_delta.as_secs_f32() + (self.acceleration * 0.5) * (time_delta.as_secs_f32() * time_delta.as_secs_f32());
        }
    }
}

impl std::fmt::Display for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Elevator (alt={:.3}m, v={:.3}m/s, a={:.3}m/s², doors={:?})", self.altitude, self.velocity, self.acceleration, self.door_state)
    }
}
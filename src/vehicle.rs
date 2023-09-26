use crate::MovementDirection;
use crate::TurnDirection;
use crate::Position;
use std::ops::Sub;

#[derive(Debug)]
pub struct Vehicle {
    pub id: i32,
    pub size: f32,
    pub  movement_direction: MovementDirection,
    pub turn_direction: TurnDirection,
    pub velocity: f32,
    pub distance_to_intersection: f32,
    pub time_to_intersection: f32,
    pub position: Position,
    pub acceleration: f32,
}

impl Vehicle {
    pub fn new(
        movement_direction: MovementDirection,
        turn_direction: TurnDirection,
        velocity: f32,
        position: Position
    ) -> Self {
        Vehicle {
            id: 0,
            size: 55.0, // Let's say every vehicle has a size of 10 units for now
            movement_direction,
            turn_direction,
            velocity,
            distance_to_intersection: 0.0,
            time_to_intersection: 0.0,
            position,
            acceleration: 0.0, // Default value
        }
    }

    pub fn update_distance_and_time_to_intersection(&mut self) {
        // Assuming intersection is at (0,0)
        self.distance_to_intersection = self.position.sub(Position::new(0.0, 0.0));
        if self.velocity != 0.0 {
            self.time_to_intersection = self.distance_to_intersection / self.velocity;
        } else {
            self.time_to_intersection = f32::MAX;
        }
    }
    pub fn update_direction_at_intersection(&mut self) {
        match self.movement_direction {
            MovementDirection::Up => {
                match self.turn_direction {
                    TurnDirection::Right => self.movement_direction = MovementDirection::Right,
                    TurnDirection::Left => self.movement_direction = MovementDirection::Left,
                    _ => {}
                }
            },
            MovementDirection::Down => {
                match self.turn_direction {
                    TurnDirection::Right => self.movement_direction = MovementDirection::Left,
                    TurnDirection::Left => self.movement_direction = MovementDirection::Right,
                    _ => {}
                }
            },
            MovementDirection::Left => {
                match self.turn_direction {
                    TurnDirection::Right => self.movement_direction = MovementDirection::Down,
                    TurnDirection::Left => self.movement_direction = MovementDirection::Up,
                    _ => {}
                }
            },
            MovementDirection::Right => {
                match self.turn_direction {
                    TurnDirection::Right => self.movement_direction = MovementDirection::Up,
                    TurnDirection::Left => self.movement_direction = MovementDirection::Down,
                    _ => {}
                }
            }
        }
    }
}
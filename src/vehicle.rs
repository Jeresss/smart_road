use crate::MovementDirection;
use crate::TurnDirection;
use crate::Position;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Lane {
    Left,
    Middle,
    Right,
}


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
    pub lane: Lane,
}

impl Vehicle {
    pub fn new(
        movement_direction: MovementDirection,
        turn_direction: TurnDirection,
        velocity: f32,
        position: Position,
        lane: Lane,
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
            lane,
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
        // Set turn direction based on the lane
        match self.lane {
            Lane::Left => self.turn_direction = TurnDirection::Left,
            Lane::Middle => self.turn_direction = TurnDirection::Straight,
            Lane::Right => self.turn_direction = TurnDirection::Right,
        }

        // Now, update the movement direction based on the turn direction
        match self.turn_direction {
            TurnDirection::Straight => {}, // No change in movement direction
            TurnDirection::Left => {
                self.movement_direction = match self.movement_direction {
                    MovementDirection::Up => MovementDirection::Left,
                    MovementDirection::Down => MovementDirection::Right,
                    MovementDirection::Left => MovementDirection::Down,
                    MovementDirection::Right => MovementDirection::Up,
                };
            },
            TurnDirection::Right => {
                self.movement_direction = match self.movement_direction {
                    MovementDirection::Up => MovementDirection::Right,
                    MovementDirection::Down => MovementDirection::Left,
                    MovementDirection::Left => MovementDirection::Up,
                    MovementDirection::Right => MovementDirection::Down,
                };
            }
        }
    }
}


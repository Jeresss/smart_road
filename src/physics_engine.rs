use crate::Vehicle;
use crate::MovementDirection;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhysicsEngine {
    safety_distance: f32,
    max_velocity: f32, // Added max velocity
}

impl PhysicsEngine {
    pub fn new(safety_distance: f32, max_velocity: f32) -> Self {
        PhysicsEngine { safety_distance, max_velocity }
    }

    // Update vehicle's position and speed based on elapsed time
    pub fn update(&self, vehicle: &mut Vehicle, elapsed_time: f32) {
        vehicle.velocity += vehicle.acceleration * elapsed_time;
        
        // Ensure velocity doesn't exceed max_velocity
        vehicle.velocity = vehicle.velocity.min(self.max_velocity);
        
        match vehicle.movement_direction {
            MovementDirection::Up => {
                vehicle.position.y -= vehicle.velocity * elapsed_time;
            },
            MovementDirection::Down => {
                vehicle.position.y += vehicle.velocity * elapsed_time;
            },
            MovementDirection::Left => {
                vehicle.position.x -= vehicle.velocity * elapsed_time;
            },
            MovementDirection::Right => {
                vehicle.position.x += vehicle.velocity * elapsed_time;
            }
        }
        
        vehicle.update_distance_and_time_to_intersection();
    }

    // Adjust vehicle's speed to maintain safety distance from vehicle ahead
    pub fn adjust_speed_for_safety(&self, vehicle: &Vehicle, vehicle_ahead: &Vehicle) -> f32 {
        let dx = vehicle_ahead.position.x - vehicle.position.x;
        let dy = vehicle_ahead.position.y - vehicle.position.y;
        let distance_to_vehicle_ahead = (dx * dx + dy * dy).sqrt();

        if distance_to_vehicle_ahead < self.safety_distance && vehicle.velocity > vehicle_ahead.velocity {
            // If the vehicle ahead is closer than the safety distance and moving slower, match its speed
            return vehicle_ahead.velocity;
        }
        return vehicle.velocity;
    }
}

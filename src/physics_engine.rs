use crate::Vehicle;
use crate::MovementDirection;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PhysicsEngine {
    safety_distance: f32,
}

impl PhysicsEngine {
    pub fn new(safety_distance: f32) -> Self {
        PhysicsEngine { safety_distance }
    }

    // Update vehicle's position and speed based on elapsed time
    pub fn update(&self, vehicle: &mut Vehicle, elapsed_time: f32) {
        vehicle.velocity += vehicle.acceleration * elapsed_time;
        
        match vehicle.movement_direction {
            MovementDirection::Up => {
                vehicle.position.y -= vehicle.velocity * elapsed_time;  // Decreasing y moves up
            },
            MovementDirection::Down => {
                vehicle.position.y += vehicle.velocity * elapsed_time;  // Increasing y moves down
            },
            MovementDirection::Left => {
                vehicle.position.x -= vehicle.velocity * elapsed_time;  // Decreasing x moves left
            },
            MovementDirection::Right => {
                vehicle.position.x += vehicle.velocity * elapsed_time;  // Increasing x moves right
            }
        }
        
        vehicle.update_distance_and_time_to_intersection();
    }
    


    // Adjust vehicle's speed to maintain safety distance from vehicle ahead
pub fn adjust_speed_for_safety(&self, vehicle: &Vehicle, vehicle_ahead: &Vehicle) -> f32 {
    let dx = vehicle_ahead.position.x - vehicle.position.x;
    let dy = vehicle_ahead.position.y - vehicle.position.y;
    let distance_to_vehicle_ahead = (dx * dx + dy * dy).sqrt();

    if distance_to_vehicle_ahead < self.safety_distance {
        return vehicle_ahead.velocity; // Return speed to match vehicle ahead
    }
    return vehicle.velocity; // Return current speed if no adjustment is needed
}   
    
}

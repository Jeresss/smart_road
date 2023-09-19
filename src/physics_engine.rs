use crate::Vehicle;

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
        let new_position = vehicle.position + vehicle.velocity * elapsed_time;
        vehicle.position = new_position;

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

// Example usage in simulation loop:
// let physics_engine = PhysicsEngine::new(5.0); // Safety distance of 5 units
// for vehicle in vehicles {
//     physics_engine.update(&mut vehicle, elapsed_time);
//     // Check if there's a vehicle ahead in the same lane and adjust speed
//     if let Some(vehicle_ahead) = get_vehicle_ahead_in_same_lane(&vehicle) {
//         physics_engine.adjust_speed_for_safety(&mut vehicle, &vehicle_ahead);
//     }
// }
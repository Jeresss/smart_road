use crate::TurnDirection;
use crate::vehicle::*;
use crate::MovementDirection;

//#[derive(Debug)]
pub struct Reservation {
    pub vehicle_id: i32,
    pub turn_direction: TurnDirection,
    pub vehicle_lane: Lane,
    pub movement_direction: MovementDirection,

    pub start_time: std::time::Duration,
    pub end_time: std::time::Duration,
}


pub struct IntersectionManager {
    reservations: Vec<Reservation>,
}


impl IntersectionManager {
    pub fn new() -> Self {
        IntersectionManager {
            reservations: Vec::new(),
        }
    }
    pub fn get_vehicle_ahead_in_same_direction(
        current_vehicle: &Vehicle,
        vehicles: &[Vehicle]
    ) -> Option<usize> {
        vehicles
            .iter()
            .enumerate()
            .filter(|&(_, v)| v.movement_direction == current_vehicle.movement_direction) // Same direction
            .filter(|&(_, v)| v.distance_to_intersection < current_vehicle.distance_to_intersection) // is ahead
            .min_by(|&(_, a), &(_, b)|
                a.distance_to_intersection.partial_cmp(&b.distance_to_intersection).unwrap()
            ) // closest vehicle ahead
            .map(|(index, _)| index) // return only the index
    }
    pub fn calculate_reservation_window(
        &self,
        vehicle: &Vehicle
    ) -> (std::time::Duration, std::time::Duration) {
        let entry_time = std::time::Duration::from_secs_f32(vehicle.time_to_intersection);
        let time_to_cross = vehicle.size / vehicle.velocity;
        let exit_time = entry_time + std::time::Duration::from_secs_f32(time_to_cross);
        (entry_time, exit_time)
    }

    pub fn request_reservation(&mut self, vehicle: &Vehicle) -> Result<(), &'static str> {
        let (start_time, end_time) = self.calculate_reservation_window(vehicle);
    
        let mut to_remove = Vec::new(); // Step 1: Create a Vec to store indices
    
        for (index, existing_reservation) in self.reservations.iter().enumerate() {
            if self.has_conflict(
                start_time,
                end_time,
                vehicle.turn_direction,
                vehicle.movement_direction,
                vehicle.lane,
                existing_reservation,
            ) {
                if start_time < existing_reservation.start_time {
                    to_remove.push(index); // Step 2: Populate the Vec
                } else {
                    return Err("Reservation conflict");
                }
            }
        }
    
        // Step 3: Remove the reservations at the collected indices
        for index in to_remove.iter().rev() {
            self.reservations.remove(*index);
        }
    
        let reservation = Reservation {
            vehicle_id: vehicle.id,
            turn_direction: vehicle.turn_direction,
            movement_direction: vehicle.movement_direction,
            vehicle_lane: vehicle.lane,
            start_time,
            end_time,
        };
        self.reservations.push(reservation);
        Ok(())
    }
    

    pub fn has_conflict(
        &self,
        proposed_start: std::time::Duration,
        proposed_end: std::time::Duration,
        proposed_turn_direction: TurnDirection,
        proposed_movement_direction: MovementDirection, 
        proposed_lane: Lane,
        existing_reservation: &Reservation
    ) -> bool {
         // Check time overlap
    if proposed_start <= existing_reservation.end_time && proposed_end >= existing_reservation.start_time {
        // Check for straight movement in opposite directions
        if proposed_turn_direction == TurnDirection::Straight && existing_reservation.turn_direction == TurnDirection::Straight {
            match (proposed_movement_direction, existing_reservation.movement_direction) {
                (MovementDirection::Up, MovementDirection::Down) | 
                (MovementDirection::Down, MovementDirection::Up) | 
                (MovementDirection::Right, MovementDirection::Left) | 
                (MovementDirection::Left, MovementDirection::Right) => {
                    return false; // No conflict for opposite straight directions
                }
                _ => {}
            }
        }
        
        // Check for left turn conflicts
        if proposed_turn_direction == TurnDirection::Left {
            return true; // Left turn conflicts with everything in the intersection
        }
        
        // Check for conflicts with left turning vehicles
        if existing_reservation.turn_direction == TurnDirection::Left {
            return true; // Anything in the intersection conflicts with a left turning vehicle
        }
        
        // ... Add any other specific conflict rules here ...

        return true; // Default to conflict for safety
    }
    false // No time overlap, so no conflict
}
}



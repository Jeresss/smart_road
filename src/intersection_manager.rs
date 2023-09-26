use crate::TurnDirection;
use crate::vehicle::Vehicle;
//#[derive(Debug)]
pub struct Reservation {
    pub vehicle_id: i32,
    pub turn_direction: TurnDirection,
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

        for existing_reservation in &self.reservations {
            if
                self.has_conflict(
                    start_time,
                    end_time,
                    vehicle.turn_direction,
                    existing_reservation
                )
            {
                return Err("Reservation conflict");
            }
        }
        let reservation = Reservation {
            vehicle_id: vehicle.id,
            turn_direction: vehicle.turn_direction,
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
        existing_reservation: &Reservation
    ) -> bool {
        // Check time overlap
        if
            proposed_start <= existing_reservation.end_time &&
            proposed_end >= existing_reservation.start_time
        {
            // Then check direction conflict based on our assumptions
            match (proposed_turn_direction, existing_reservation.turn_direction) {
                (TurnDirection::Left, _) => {
                    return true;
                } // Left conflicts with everything
                (_, TurnDirection::Left) => {
                    return true;
                } // Anything conflicts with left
                (TurnDirection::Straight, TurnDirection::Straight) => {
                    // If they are coming from opposite directions, they conflict
                    // This would require further details of the direction they're coming from which is not provided currently
                    // For now, let's assume they always conflict to be safe.
                    return true;
                }
                // Add more rules if needed
                _ => {
                    return false;
                } // No conflict for other cases
            }
        }
        false
    }
}



extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod physics_engine;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TurnDirection {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

use std::ops::Add;
impl Add<f32> for Position {
    type Output = Position;

    fn add(self, other: f32) -> Position {
        Position { x: self.x + other, y: self.y + other }
    }
}

use std::ops::Sub;
use physics_engine::PhysicsEngine;
impl Sub for Position {
    type Output = f32;

    fn sub(self, other: Position) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }
}

#[derive(Debug)]
pub struct Vehicle {
    id: u32,
    movement_direction: MovementDirection,
    turn_direction: TurnDirection,
    velocity: f32,
    distance_to_intersection: f32,
    time_to_intersection: f32,
    position: Position,
}

impl Vehicle {
    pub fn new(movement_direction: MovementDirection, turn_direction: TurnDirection, velocity: f32, position: Position) -> Self {
        Vehicle {
            id: 0,
            movement_direction,
            turn_direction,
            velocity,
            distance_to_intersection: 0.0,
            time_to_intersection: 0.0,
            position,
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
}

#[derive(Debug)]
pub struct Reservation {
    vehicle_id: u32,
    turn_direction: TurnDirection,
    reserved_time: std::time::Duration,
}

struct IntersectionManager {
    reservations: Vec<Reservation>,
}

impl IntersectionManager {
    fn new() -> Self {
        IntersectionManager {
            reservations: Vec::new(),
        }
    }

    fn request_reservation(&mut self, vehicle: &Vehicle) -> Result<(), &'static str> {
        let reservation_time = std::time::Duration::from_secs_f32(vehicle.time_to_intersection);
        for existing_reservation in &self.reservations {
            if self.has_conflict(reservation_time.as_secs_f32(), vehicle.turn_direction, existing_reservation) {
                return Err("Reservation conflict");
            }
        }
        let reservation = Reservation {
            vehicle_id: vehicle.id,
            turn_direction: vehicle.turn_direction,
            reserved_time: reservation_time,
        };
        self.reservations.push(reservation);
        Ok(())
    }

    fn has_conflict(&self, proposed_time: f32, proposed_turn_direction: TurnDirection, existing_reservation: &Reservation) -> bool {
        let existing_turn_direction = existing_reservation.turn_direction;
        let time_difference = (proposed_time - existing_reservation.reserved_time.as_secs_f32()).abs();
        if time_difference < 1.0 {
            return match (proposed_turn_direction, existing_turn_direction) {
                (TurnDirection::Left, TurnDirection::Left) => true,
                (TurnDirection::Straight, TurnDirection::Straight) => true,
                (TurnDirection::Left, TurnDirection::Straight) => true,
                (TurnDirection::Straight, TurnDirection::Left) => true,
                (TurnDirection::Right, TurnDirection::Right) => true,
                _ => false
            };
        }
        false
    }
}

    
pub fn get_vehicle_ahead_in_same_direction(current_vehicle: &Vehicle, vehicles: &[Vehicle]) -> Option<usize> {
    vehicles.iter().enumerate()
        .filter(|&(_, v)| v.movement_direction == current_vehicle.movement_direction) // Same direction
        .filter(|&(_, v)| v.distance_to_intersection < current_vehicle.distance_to_intersection) // is ahead
        .min_by(|&(_, a), &(_, b)| a.distance_to_intersection.partial_cmp(&b.distance_to_intersection).unwrap()) // closest vehicle ahead
        .map(|(index, _)| index) // return only the index
}



fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Traffic Simulation", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vehicles = Vec::new();
    let mut im = IntersectionManager::new();
    let physics_engine = PhysicsEngine::new(5.0); // Safety distance of 5 units
    let mut next_vehicle_id: u32 = 1;

    const SIMULATION_DURATION: usize = 1000;
    const VEHICLE_CREATION_PROBABILITY: f32 = 0.05;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: 600.0 },
                        velocity: 10.0,
                        movement_direction: MovementDirection::Up,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: 0,
                    };
                    vehicles.push(vehicle);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: 600.0 },
                        velocity: 10.0,
                        movement_direction: MovementDirection::Down,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: 0,
                    };
                    vehicles.push(vehicle);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: 600.0 },
                        velocity: 10.0,
                        movement_direction: MovementDirection::Left,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: 0,
                    };
                    vehicles.push(vehicle);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: 600.0 },
                        velocity: 10.0,
                        movement_direction: MovementDirection::Right,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: 0,
                    };
                    vehicles.push(vehicle);
                },
                _ => {},
            }
        }

        if rand::random::<f32>() < VEHICLE_CREATION_PROBABILITY {
            let turn_direction = if rand::random::<f32>() < 0.33 {
                TurnDirection::Left
            } else if rand::random::<f32>() < 0.5 {
                TurnDirection::Straight
            } else {
                TurnDirection::Right
            };
        
            let movement_direction = match rand::random::<f32>() {
                x if x < 0.25 => MovementDirection::Up,
                x if x < 0.5 => MovementDirection::Down,
                x if x < 0.75 => MovementDirection::Left,
                _ => MovementDirection::Right,
            };
        
            let position = match movement_direction {
                MovementDirection::Up => Position::new(400.0, -100.0), // assuming center x position
                MovementDirection::Down => Position::new(400.0, 800.0), // assuming center x position
                MovementDirection::Left => Position::new(800.0, 400.0), // assuming center y position
                MovementDirection::Right => Position::new(-100.0, 400.0), // assuming center y position
            };
        
            vehicles.push(Vehicle::new(movement_direction, turn_direction, 30.0, position));
            vehicles.last_mut().unwrap().id = next_vehicle_id;
            next_vehicle_id += 1;
        }
        
        

        let mut adjustments: Vec<(usize, f32)> = Vec::new(); // Stores (index, new_speed)
        let mut vehicle_pairs: Vec<(usize, usize)> = Vec::new();

        for i in 0..vehicles.len() {
            physics_engine.update(&mut vehicles[i], 1.0);
            vehicles[i].update_distance_and_time_to_intersection();

            if let Some(vehicle_ahead_index) = get_vehicle_ahead_in_same_direction(&vehicles[i], &vehicles) {
                vehicle_pairs.push((i, vehicle_ahead_index));
            }

            if vehicles[i].distance_to_intersection < 20.0 && vehicles[i].time_to_intersection > 0.0 {
                match im.request_reservation(&vehicles[i]) {
                    Ok(_) => {
                        println!("Reservation granted for vehicle {}", vehicles[i].id);
                    },
                    Err(e) => {
                        println!("Reservation error for vehicle {}: {}", vehicles[i].id, e);
                        vehicles[i].velocity = 0.0;
                    }
                }
            }
        }

        for (vehicle_index, vehicle_ahead_index) in vehicle_pairs {
            let new_speed = physics_engine.adjust_speed_for_safety(&vehicles[vehicle_index], &vehicles[vehicle_ahead_index]);
            adjustments.push((vehicle_index, new_speed));
        }

        for (index, new_speed) in adjustments {
            vehicles[index].velocity = new_speed;
        }

        vehicles.retain(|vehicle| vehicle.distance_to_intersection > 0.0);

        // Ideally, after the update, you would also have code to render the vehicles.
    }
}

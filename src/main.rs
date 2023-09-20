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
    id: i32,
    size: f32,
    movement_direction: MovementDirection,
    turn_direction: TurnDirection,
    velocity: f32,
    distance_to_intersection: f32,
    time_to_intersection: f32,
    position: Position,
    acceleration: f32,
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
            size: 10.0, // Let's say every vehicle has a size of 10 units for now
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
}

//#[derive(Debug)]
pub struct Reservation {
    vehicle_id: i32,
    turn_direction: TurnDirection,
    start_time: std::time::Duration,
    end_time: std::time::Duration,
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
    fn calculate_reservation_window(
        &self,
        vehicle: &Vehicle
    ) -> (std::time::Duration, std::time::Duration) {
        let entry_time = std::time::Duration::from_secs_f32(vehicle.time_to_intersection);
        let time_to_cross = vehicle.size / vehicle.velocity;
        let exit_time = entry_time + std::time::Duration::from_secs_f32(time_to_cross);
        (entry_time, exit_time)
    }

    fn request_reservation(&mut self, vehicle: &Vehicle) -> Result<(), &'static str> {
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

    fn has_conflict(
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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Traffic Simulation", 1000, 1000)
    .position_centered()
    .build()
    .unwrap();


    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut vehicles = Vec::new();
    let mut im = IntersectionManager::new();
    let physics_engine = PhysicsEngine::new(5.0); // Safety distance of 5 units
    let mut next_vehicle_id: i32 = 1;

    const VEHICLE_CREATION_PROBABILITY: f32 = 0.05;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: 867.0 },
                        size: 10.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Up,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 400.0, y: -67.0 },
                        size: 10.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Down,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: 867.0, y: 400.0 },
                        size: 10.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Left,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let vehicle = Vehicle {
                        position: Position { x: -67.0, y: 400.0 },
                        size: 10.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Right,
                        turn_direction: TurnDirection::Straight,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                _ => {}
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
                MovementDirection::Up => {
                    let x = match turn_direction {
                        TurnDirection::Right => 400.0 - 22.0,
                        TurnDirection::Straight => 400.0,
                        TurnDirection::Left => 400.0 + 22.0,
                    };
                    Position::new(x, 867.0)
                }
                MovementDirection::Down => {
                    let x = match turn_direction {
                        TurnDirection::Right => 400.0 + 22.0,
                        TurnDirection::Straight => 400.0,
                        TurnDirection::Left => 400.0 - 22.0,
                    };
                    Position::new(x, -67.0)
                }
                MovementDirection::Left => {
                    let y = match turn_direction {
                        TurnDirection::Right => 400.0 + 22.0,
                        TurnDirection::Straight => 400.0,
                        TurnDirection::Left => 400.0 - 22.0,
                    };
                    Position::new(867.0, y)
                }
                MovementDirection::Right => {
                    let y = match turn_direction {
                        TurnDirection::Right => 400.0 - 22.0,
                        TurnDirection::Straight => 400.0,
                        TurnDirection::Left => 400.0 + 22.0,
                    };
                    Position::new(-67.0, y)
                }
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

            if
                let Some(vehicle_ahead_index) = get_vehicle_ahead_in_same_direction(
                    &vehicles[i],
                    &vehicles
                )
            {
                vehicle_pairs.push((i, vehicle_ahead_index));
            }

            if
                vehicles[i].distance_to_intersection < 20.0 &&
                vehicles[i].time_to_intersection > 0.0
            {
                match im.request_reservation(&vehicles[i]) {
                    Ok(_) => {
                        println!("Reservation granted for vehicle {}", vehicles[i].id);
                    }
                    Err(e) => {
                        println!("Reservation error for vehicle {}: {}", vehicles[i].id, e);
                        vehicles[i].velocity = 0.0;
                    }
                }
            }
        }

        for (vehicle_index, vehicle_ahead_index) in vehicle_pairs {
            let new_speed = physics_engine.adjust_speed_for_safety(
                &vehicles[vehicle_index],
                &vehicles[vehicle_ahead_index]
            );
            adjustments.push((vehicle_index, new_speed));
        }

        for (index, new_speed) in adjustments {
            vehicles[index].velocity = new_speed;
        }

        vehicles.retain(|vehicle| {
            match vehicle.movement_direction {
                MovementDirection::Up => vehicle.position.y >= 0.0,
                MovementDirection::Down => vehicle.position.y <= 800.0,
                MovementDirection::Left => vehicle.position.x >= 0.0,
                MovementDirection::Right => vehicle.position.x <= 800.0,
            }
        });

           // Clear canvas with a green background.
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 128, 0)); // Green color
            canvas.clear();

        draw_grid(&mut canvas);
        draw_roads(&mut canvas);
        draw_center_lines(&mut canvas);
        draw_intersection(&mut canvas);

        // Draw vehicles
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        for vehicle in &vehicles {
            let x = (vehicle.position.x as i32) - 5;
            let y = (vehicle.position.y as i32) - 5;
            canvas.fill_rect(sdl2::rect::Rect::new(x, y, 10, 10)).unwrap(); // Vehicles are 10x10 squares
        }
        canvas.present(); // Present the rendered frame

        std::thread::sleep(std::time::Duration::from_millis(16)); // Delay for ~60 FPS
    }
}

fn draw_grid(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0)); // Black color for grid lines
    
    for i in (0..=1000).step_by(56) {
        // Vertical lines
        canvas.draw_line((i, 0), (i, 1000)).unwrap();
        
        // Horizontal lines
        canvas.draw_line((0, i), (1000, i)).unwrap();
    }
}

fn draw_roads(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100)); // Gray color for roads
    
    // Vertical roads
    canvas.fill_rect(sdl2::rect::Rect::new(6 * 56, 0, 6 * 56, 1000)).unwrap();
    
    // Horizontal roads
    canvas.fill_rect(sdl2::rect::Rect::new(0, 6 * 56, 1000, 6 * 56)).unwrap();
}

fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255)); // White color for intersection
    
    canvas.fill_rect(sdl2::rect::Rect::new(6 * 56, 6 * 56, 6 * 56, 6 * 56)).unwrap();
}

fn draw_center_lines(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 0)); // Yellow color for center lines

    let dash_length = 28; // half of the cell size
    let space_length = 28; // another half of the cell size
    let line_thickness = 4; // chosen thickness for the yellow center lines

    let mut start_y = 0;
    while start_y < 1000 {
        // Vertical center lines
        canvas.fill_rect(sdl2::rect::Rect::new((9 * 56 - line_thickness) as i32, start_y as i32, line_thickness as u32, dash_length)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new((9 * 56 + line_thickness) as i32, start_y as i32, line_thickness as u32, dash_length)).unwrap();
                
        // Horizontal center lines
        canvas.fill_rect(sdl2::rect::Rect::new(start_y as i32, (9 * 56 - line_thickness) as i32, dash_length, line_thickness as u32)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(start_y as i32, (9 * 56 + line_thickness) as i32, dash_length, line_thickness as u32)).unwrap();
        
        

        start_y += dash_length + space_length;
    }
}
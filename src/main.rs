extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use vehicle::Vehicle;
use intersection_manager::IntersectionManager;

 mod vehicle;
 mod intersection_manager;
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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    let turn_direction;
                    let x = if rand::random::<f32>() < 0.33 {
                        turn_direction = TurnDirection::Right;
                        9.75 * 56.0
                    } else if rand::random::<f32>() < 0.5 {
                        turn_direction = TurnDirection::Straight;
                        10.7 * 56.0
                    } else {
                        turn_direction = TurnDirection::Left;
                        11.5 * 56.0
                    };
                    let vehicle = Vehicle {
                        position: Position { x, y: 867.0 },
                        size: 55.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Up,
                        turn_direction,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    let turn_direction = if rand::random::<f32>() < 0.33 {
                        TurnDirection::Left
                    } else if rand::random::<f32>() < 0.5 {
                        TurnDirection::Straight
                    } else {
                        TurnDirection::Right
                    };
                
                    let x = match turn_direction {
                        TurnDirection::Right => 6.6 * 56.0,
                        TurnDirection::Straight => 7.6 * 56.0,
                        TurnDirection::Left => 8.6 * 56.0,
                    };
                    let vehicle = Vehicle {
                        position: Position { x, y: -67.0 },
                        size: 55.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Down,
                        turn_direction,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    let turn_direction = if rand::random::<f32>() < 0.33 {
                        TurnDirection::Left
                    } else if rand::random::<f32>() < 0.5 {
                        TurnDirection::Straight
                    } else {
                        TurnDirection::Right
                    };
                
                    let y = match turn_direction {
                        TurnDirection::Right => 6.6 * 56.0,
                        TurnDirection::Straight => 7.6 * 56.0,
                        TurnDirection::Left => 8.6 * 56.0,
                    };
                    let vehicle = Vehicle {
                        position: Position { x: 867.0, y },
                        size: 55.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Left,
                        turn_direction,
                        distance_to_intersection: 0.0,
                        time_to_intersection: 0.0,
                        id: next_vehicle_id,
                        acceleration: 0.0,
                    };
                    vehicles.push(vehicle);
                    next_vehicle_id += 1;
                }
                
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    let turn_direction = if rand::random::<f32>() < 0.33 {
                        TurnDirection::Left
                    } else if rand::random::<f32>() < 0.5 {
                        TurnDirection::Straight
                    } else {
                        TurnDirection::Right
                    };
                    
                    let y = match turn_direction {
                        TurnDirection::Left => 9.5 * 56.0,
                        TurnDirection::Straight => 10.5 * 56.0,
                        TurnDirection::Right => 11.2 * 56.0,
                    };
                    let vehicle = Vehicle {
                        position: Position { x: -67.0, y },
                        size: 55.0,
                        velocity: 10.0,
                        movement_direction: MovementDirection::Right,
                        turn_direction,
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

        let mut adjustments: Vec<(usize, f32)> = Vec::new(); // Stores (index, new_speed)
        let mut vehicle_pairs: Vec<(usize, usize)> = Vec::new();

        for i in 0..vehicles.len() {
            physics_engine.update(&mut vehicles[i], 1.0);
            vehicles[i].update_distance_and_time_to_intersection();
            println!("Vehicle {} distance to intersection: {}", vehicles[i].id, vehicles[i].distance_to_intersection);


                // Check if vehicle is at the intersection
                if vehicles[i].distance_to_intersection < 5.0 && vehicles[i].distance_to_intersection > 0.0 {
                    println!("Vehicle {} reached intersection with turn direction: {:?}", vehicles[i].id, vehicles[i].turn_direction);
                    vehicles[i].update_direction_at_intersection();
                    println!("Vehicle {} new movement direction: {:?}", vehicles[i].id, vehicles[i].movement_direction);
                }
                

                if let Some(vehicle_ahead_index) = IntersectionManager::get_vehicle_ahead_in_same_direction(&vehicles[i], &vehicles) {
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
                MovementDirection::Down => vehicle.position.y <= 1000.0,
                MovementDirection::Left => vehicle.position.x >= 0.0,
                MovementDirection::Right => vehicle.position.x <= 1000.0,
            }
        });

           // Clear canvas with a green background.
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 128, 0)); // Green color
            canvas.clear();

       // draw_grid(&mut canvas);
        draw_roads(&mut canvas);
        draw_boundary_lines(&mut canvas);
        draw_center_lines(&mut canvas);
        draw_intersection(&mut canvas);

        // Draw vehicles
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        for vehicle in &vehicles {
            let x = (vehicle.position.x as i32) - (vehicle.size as i32 / 2);
            let y = (vehicle.position.y as i32) - (vehicle.size as i32 / 2);
            canvas.fill_rect(sdl2::rect::Rect::new(x, y, vehicle.size as u32,vehicle.size as u32)).unwrap(); // Vehicles are 10x10 squares
        }
        canvas.present(); // Present the rendered frame

        std::thread::sleep(std::time::Duration::from_millis(16)); // Delay for ~60 FPS
    }
}

/* 
fn draw_grid(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0)); // Black color for grid lines
    
    for i in (0..=1000).step_by(56) {
        // Vertical lines
        canvas.draw_line((i, 0), (i, 1000)).unwrap();
        
        // Horizontal lines
        canvas.draw_line((0, i), (1000, i)).unwrap();
    }
}
 */
fn draw_roads(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100)); // Gray color for roads
    
    // Vertical roads
    canvas.fill_rect(sdl2::rect::Rect::new(6 * 56, 0, 6 * 56, 1000)).unwrap();
    
    // Horizontal roads
    canvas.fill_rect(sdl2::rect::Rect::new(0, 6 * 56, 1000, 6 * 56)).unwrap();
}

fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100)); // Gray color for roads
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

fn draw_boundary_lines(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0)); // Black color for boundary lines

    let thickness: i32 = 4; // Adjusted thickness of boundary lines

    // Vertical boundaries
    canvas.fill_rect(sdl2::rect::Rect::new(6 * 56, 0, thickness as u32, 1000)).unwrap(); // Left boundary
    canvas.fill_rect(sdl2::rect::Rect::new(12 * 56 - thickness, 0, thickness as u32, 1000)).unwrap(); // Right boundary

    // Horizontal boundaries - Upper
    canvas.fill_rect(sdl2::rect::Rect::new(0, 6 * 56 - thickness, 6 * 56, thickness as u32)).unwrap(); // Left part
    canvas.fill_rect(sdl2::rect::Rect::new(12 * 56, 6 * 56 - thickness, 1000 - 12 * 56, thickness as u32)).unwrap(); // Right part

    // Horizontal boundaries - Lower
    canvas.fill_rect(sdl2::rect::Rect::new(0, 12 * 56, 6 * 56, thickness as u32)).unwrap(); // Left part
    canvas.fill_rect(sdl2::rect::Rect::new(12 * 56, 12 * 56, 1000 - 12 * 56, thickness as u32)).unwrap(); // Right part

    // Dashed lines for non-turning sections on intersection
    let dash_length: i32 = 28; // half of the cell size
    let space_length: i32 = 28; // another half of the cell size
    let line_thickness: i32 = 4; // chosen thickness for the dashed lines

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255)); // White color for dashed lines
    
    let mut start_pos = 6 * 56;
    while start_pos < 12 * 56 {
        // Top line
        canvas.fill_rect(sdl2::rect::Rect::new(start_pos, 6 * 56 - line_thickness, dash_length as u32, line_thickness as u32)).unwrap();
        
        // Bottom line
        canvas.fill_rect(sdl2::rect::Rect::new(start_pos, 12 * 56, dash_length as u32, line_thickness as u32)).unwrap();

        start_pos += dash_length + space_length;
    }

    start_pos = 6 * 56;
    while start_pos < 12 * 56 {
        // Left line
        canvas.fill_rect(sdl2::rect::Rect::new(6 * 56 - line_thickness, start_pos, line_thickness as u32, dash_length as u32)).unwrap();

        // Right line
        canvas.fill_rect(sdl2::rect::Rect::new(12 * 56, start_pos, line_thickness as u32, dash_length as u32)).unwrap();
        
        start_pos += dash_length + space_length;
    }
}

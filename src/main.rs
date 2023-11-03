use core::time;
use std::{time::{Instant, Duration}, sync::RwLockWriteGuard};

use console_engine::{screen::Screen, pixel, Color, ConsoleEngine, KeyCode};


const WIDTH: i32 = 30;
const HEIGHT: i32 = 30;
const RECT_LOWER_X: i32 = WIDTH - 1;
const RECT_LOWER_Y: i32 = HEIGHT - 1;


struct Shape {
    shape: &'static str,
    x: i32,
    y: i32,
    len: i32,
}

impl Shape {
    fn new(shape: &'static str, x: i32, y: i32) -> Shape {
        Shape { shape, x, y, len: shape.chars().count() as i32 }
    }

    fn bar() -> Shape {
        Shape::new("      ", 3, 3)
    }
}


fn main() {

    let mut engine = ConsoleEngine::init(WIDTH as u32, HEIGHT as u32, 80).unwrap();
    let mut stopwatch = Instant::now();
    let mut old_frame_count = 0;
    let mut elapsed_frames = 0;

    let mut inactive_shapes: Vec<Shape> = vec![];
    let mut active_shape = Shape::bar();

    loop {
        engine.wait_frame();
        engine.clear_screen();
        
        // define box
        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg(' ', Color::White));

        // print inactive shapes
        for shape in &inactive_shapes {
            engine.print_fbg(shape.x, shape.y, shape.shape, Color::Blue, Color::Blue);
        }

        // print active shape
        engine.print_fbg(active_shape.x, active_shape.y, active_shape.shape, Color::Blue, Color::Blue);

        // go down one unit per second or if s is pressed
        if engine.frame_count % 60 == 0 || engine.is_key_pressed(KeyCode::Char('s')) {
            if !has_collision(&inactive_shapes, &active_shape, "y+") {
                active_shape.y += 1;
            } else {
                // game over if stack too high
                if active_shape.y == 3 {
                    break
                }

                // hit something? -> retire active shape
                inactive_shapes.push(active_shape);
    
                // generate new active shape
                active_shape = Shape::bar();
            }
        }

        // debug coordinates and shape counter
        engine.print(10, 0, format!("Y:{} X:{}", active_shape.y, active_shape.x).as_str());
        engine.print(23, 0, format!("{}", inactive_shapes.len()).as_str());

        // FPS counter
        if stopwatch.elapsed().as_millis() >= Duration::from_millis(1000).as_millis() {
            elapsed_frames = engine.frame_count - old_frame_count;

            // reset counters
            old_frame_count = engine.frame_count;
            stopwatch = Instant::now()
        }
        engine.print(0, 0, format!("{} FPS", elapsed_frames).as_str());
        

        // press c to exit app
        if engine.is_key_pressed(KeyCode::Char('c')) {
            break
        }

        // LEFT
        else if engine.is_key_pressed(KeyCode::Char('q')) {
            if !has_collision(&inactive_shapes, &active_shape, "x-") {
                active_shape.x -= 1;
            }
        }

        // RIGHT
        else if engine.is_key_pressed(KeyCode::Char('d')) {
            if !has_collision(&inactive_shapes, &active_shape, "x+") {
                active_shape.x += 1;
            }
        }
    

        engine.draw();
    }

    
}

fn has_collision(inactive_shapes: &Vec<Shape>, active_shape: &Shape, direction_change: &str) -> bool {
    match direction_change {
        // move right
        "x+" => {
            // hit right wall
            if active_shape.x + active_shape.len >= RECT_LOWER_X {
                return true
            }

            // 2 shapes can't have overlaying cells
            for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x+active_shape.len == inactive_shape.x && active_shape.y == inactive_shape.y {
                            return true
                        }   
                    }
                }
            }
        },
        // move left
        "x-" => {
            if active_shape.x-1 <= 0 {
                return true
            }

            // 2 shapes can't have overlaying cells
            for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x-1 == inactive_shape.x+inactive_shape.len-1 && active_shape.y == inactive_shape.y {
                            return true
                        }   
                    }
                }
            }
        },
        // move down
        "y+" => {
            // hit bottom
            if active_shape.y+1 >= RECT_LOWER_Y {
                return true
            }

            // 2 shapes can't have overlaying cells
            for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x + i == inactive_shape.x + j && active_shape.y+1 == inactive_shape.y {
                            return true
                        }   
                    }
                }
            }
            
        },
        _ => panic!("unknown direction change")
    }

    false
}

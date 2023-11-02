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
}

impl Shape {
    fn new(shape: &'static str, x: i32, y: i32) -> Shape {
        Shape { shape, x, y }
    }
}


fn main() {

    let mut engine = ConsoleEngine::init(WIDTH as u32, HEIGHT as u32, 80).unwrap();
    let mut stopwatch = Instant::now();
    let mut old_frame_count = 0;
    let mut elapsed_frames = 0;

    let mut inactive_shapes: Vec<Shape> = vec![];
    let mut active_shape = Shape::new("####", 3, 3);

    loop {
        engine.wait_frame();
        engine.clear_screen();

        
        // define box
        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg(' ', Color::White));


        // print inactive shapes
        for shape in &inactive_shapes {
            engine.print(shape.x, shape.y, shape.shape);
        }

        // print active shape
        engine.print(active_shape.x, active_shape.y, active_shape.shape);

        // go down one unit per second
        if engine.frame_count % 60 == 0 {
            if active_shape.y+1 < RECT_LOWER_Y {
                active_shape.y += 1;
            } else {
                // hit something? -> retire active shape
                inactive_shapes.push(active_shape);
    
                // generate new active shape
                active_shape = Shape::new("####", 3, 3)
            }
        }
        

        // debug coordinates
        engine.print(10, 0, format!("Y:{} X:{}", active_shape.y, active_shape.x).as_str());

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

        else if engine.is_key_pressed(KeyCode::Char('s')) {
            if active_shape.y+1 < RECT_LOWER_Y {
                active_shape.y += 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('z')) {
            if active_shape.y-1 > 1 {
                active_shape.y -= 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('q')) {
            if active_shape.x-1 > 0 {
                active_shape.x -= 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('d')) {
            if active_shape.x + (active_shape.shape.len() as i32) < RECT_LOWER_X {
                active_shape.x += 1;
            }
        }


        engine.draw();
    }

    
}

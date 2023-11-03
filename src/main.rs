use core::time;
use std::{time::{Instant, Duration}, sync::RwLockWriteGuard, ops::Index};

use console_engine::{screen::Screen, pixel, Color, ConsoleEngine, KeyCode};


const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const RECT_LOWER_X: i32 = 30;
const RECT_LOWER_Y: i32 = 30;


struct Shape {
    shape: &'static str,
    x: i32,
    y: i32,
    slices: Vec<Slice>,
}

impl Shape {
    fn new(shape: &'static str, x: i32, y: i32) -> Shape {
        let default_x = 3;
        let default_y = 3;
        let mut slices: Vec<Slice> = vec![];
        for (y_offset, slice) in shape.split('\n').enumerate() {
            // calculate x axis offset for #
            let x_offset = (slice.chars().position(|c| c == '#').unwrap()) as i32;

            slices.push(Slice::new(default_x+x_offset, default_y+(y_offset as i32), slice.trim()));
        }
        Shape { shape, x, y, slices }
    }

    fn bar() -> Shape {
        let l_shape = "#####\r\
                    # \n\
                    # \n\
                    # ";
        Shape::new(l_shape, 3, 3)

    }

    fn l_shape() -> Shape {
        let l_shape = "    #\n\
                       #####";
        let l_shape = "###\n  ###";
        
        Shape::new(l_shape, 3, 3)
    }

    fn go_down(&mut self) {
        for slice in &mut self.slices {
            slice.y += 1;
        }
    }

    fn go_left(&mut self) {
        for slice in &mut self.slices {
            slice.x -= 1;
        }
    }

    fn go_right(&mut self) {
        for slice in &mut self.slices {
            slice.x += 1;
        }
    }
}

struct Slice {
    x: i32, // start where first # is found
    y: i32, // different y pos per slice
    filling: &'static str,
}

impl Slice {
    fn new(x: i32, y: i32, filling: &'static str) -> Slice {
        Slice { x, y, filling }
    }
}


fn main() {

    let mut engine = ConsoleEngine::init(WIDTH as u32, HEIGHT as u32, 80).unwrap();
    let mut stopwatch = Instant::now();
    let mut old_frame_count = 0;
    let mut elapsed_frames = 0;

    let mut inactive_shapes: Vec<Shape> = vec![];
    let mut active_shape = Shape::l_shape();

    loop {
        engine.wait_frame();
        engine.clear_screen();
        
        // define box
        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg(' ', Color::White));

        // print inactive shapes
        for shape in &inactive_shapes {
            for slice in &shape.slices {
                engine.print_fbg(slice.x, slice.y, slice.filling, Color::White, Color::Blue);
            }
        }

        // print active shape
        for slice in &active_shape.slices {
            engine.print_fbg(slice.x, slice.y, slice.filling, Color::White, Color::Blue);
        }

        // go down one unit per second or if s is pressed
        if engine.frame_count % 60 == 0 || engine.is_key_pressed(KeyCode::Char('s')) {
            if !has_collision(&inactive_shapes, &active_shape, "y+") {
                active_shape.go_down();
            } else {
                // game over if stack too high
                if active_shape.slices.last().unwrap().y == 3 {
                    break
                }

                // hit something? -> retire active shape
                inactive_shapes.push(active_shape);
    
                // generate new active shape
                active_shape = Shape::l_shape();
            }
        }

        // debug coordinates and shape counter
        engine.print(10, 0, format!("Y:{} X:{}", active_shape.y, active_shape.x).as_str());
        engine.print(23, 0, format!("{}", active_shape.slices[0].filling.chars().count()).as_str());

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
                active_shape.go_left()
            }
        }

        // RIGHT
        else if engine.is_key_pressed(KeyCode::Char('d')) {
            if !has_collision(&inactive_shapes, &active_shape, "x+") {
                active_shape.go_right()
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
            for slice in active_shape.slices.iter() {
                if slice.x + slice.filling.chars().count() as i32 >= RECT_LOWER_X {
                    return true
                }
            }

            // 2 shapes can't have overlaying cells
            /* for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x+active_shape.len == inactive_shape.x && active_shape.y == inactive_shape.y {
                            return true
                        }   
                    }
                }
            } */
        },
        // move left
        "x-" => {
            // hit left wall
            for slice in active_shape.slices.iter() {
                if slice.x - 1 <= 0 {
                    return true
                }
            }

            // 2 shapes can't have overlaying cells
            /* for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x-1 == inactive_shape.x+inactive_shape.len-1 && active_shape.y == inactive_shape.y {
                            return true
                        }   
                    }
                }
            } */
        },
        // move down
        "y+" => {
            // hit bottom
            for slice in active_shape.slices.iter() {
                if slice.y + 1 >= RECT_LOWER_Y {
                    return true
                }
            }

            // 2 shapes can't have overlaying cells
            /* for inactive_shape in inactive_shapes {
                for i in 0..active_shape.len {
                    for j in 0..inactive_shape.len {
                        // can't have shared x pos on the same y pos
                        if active_shape.x + i == inactive_shape.x + j && active_shape.y+1 == inactive_shape.y {
                            return true
                        }   
                    }
                }
            } */
            
        },
        _ => panic!("unknown direction change")
    }

    false
}

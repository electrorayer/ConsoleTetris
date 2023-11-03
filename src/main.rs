use core::time;
use std::{time::{Instant, Duration}, sync::RwLockWriteGuard, ops::Index, cell};

use console_engine::{screen::Screen, pixel, Color, ConsoleEngine, KeyCode};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};


const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const RECT_LOWER_X: i32 = 30;
const RECT_LOWER_Y: i32 = 30;


struct Shape {
    slices: Vec<Slice>,
    color:  Color,
}

impl Shape {
    fn new(shape: &'static str) -> Shape {
        let colors = vec![Color::DarkGrey, Color::Red, Color::DarkRed, Color::Green, Color::DarkGreen, Color::Yellow, Color::DarkYellow, Color::Blue, Color::DarkBlue, Color::Magenta, Color::DarkMagenta, Color::Cyan, Color::DarkCyan];
        
        let default_x = 3;
        let default_y = 3;
        let mut slices: Vec<Slice> = vec![];
        for (y_offset, slice) in shape.split('\n').enumerate() {
            // calculate x axis offset for #
            let x_offset = (slice.chars().position(|c| c == '#').unwrap()) as i32;

            slices.push(Slice::new(default_x+x_offset, default_y+(y_offset as i32), slice.trim()));
        }

        let rand_color = colors.choose(&mut rand::thread_rng()).unwrap().clone();
        Shape { slices, color: rand_color} 
    }

    fn bar() -> Shape {
        let l_shape = "##\n##\n##\n##\n##";
        Shape::new(l_shape)
    }

    fn cube() -> Shape {
        let l_shape = "####\n####";
        Shape::new(l_shape)
    }

    fn z_shape() -> Shape {
        let l_shape = "####\n  ####";
        Shape::new(l_shape)
    }

    fn s_shape() -> Shape {
        let l_shape = "  ####\n####";
        Shape::new(l_shape)
    }

    fn l_shape() -> Shape {
        let l_shape = "    ##\n######";
        Shape::new(l_shape)
    }

    fn j_shape() -> Shape {
        let l_shape = "######\n    ##";
        Shape::new(l_shape)
    }

    fn t_shape() -> Shape {
        let l_shape = "######\n  ##  ";
        Shape::new(l_shape)
    }

    fn random_shape(rng: &mut ThreadRng) -> Shape {
        let i = rng.gen_range(0..7);
        return match i {
            0 => Shape::bar(),
            1 => Shape::cube(),
            2 => Shape::z_shape(),
            3 => Shape::s_shape(),
            4 => Shape::l_shape(),
            5 => Shape::j_shape(),
            6 => Shape::t_shape(),
            _ => panic!("Impossible lol"),
        };
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

    let mut rng = rand::thread_rng();
    let mut engine = ConsoleEngine::init(WIDTH as u32, HEIGHT as u32, 80).unwrap();
    let mut stopwatch = Instant::now();
    let mut old_frame_count = 0;
    let mut elapsed_frames = 0;

    let mut inactive_shapes: Vec<Shape> = vec![];
    let mut active_shape = Shape::random_shape(&mut rng);

    loop {
        engine.wait_frame();
        engine.clear_screen();
        
        // define box
        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg(' ', Color::White));

        // print inactive shapes
        for shape in &inactive_shapes {
            for slice in &shape.slices {
                engine.print_fbg(slice.x, slice.y, slice.filling, shape.color, shape.color);
            }
        }

        // print active shape
        for slice in &active_shape.slices {
            engine.print_fbg(slice.x, slice.y, slice.filling, active_shape.color, active_shape.color);
        }

        // go down one unit per second or if s is pressed
        if engine.frame_count % 60 == 0 || engine.is_key_pressed(KeyCode::Char('s')) {
            if !has_collision(&inactive_shapes, &active_shape, "y+") {
                active_shape.go_down();
            } else {
                // game over if stack too high
                if active_shape.slices.first().unwrap().y == 3 {
                    break
                }

                // hit something? -> retire active shape
                inactive_shapes.push(active_shape);
    
                // generate new active shape
                active_shape = Shape::random_shape(&mut rng)
                
            }
        }

        // debug coordinates
        // engine.print(10, 0, format!("Y:{} X:{}", active_shape.y, active_shape.x).as_str());

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

        // change rotation
        if engine.is_key_pressed(KeyCode::Char('a')) {
            // change rotation
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

                for (i, _) in slice.filling.chars().enumerate() {
                    let cell_x_pos = slice.x + i as i32;
                    for inactive_shape in inactive_shapes {
                        for inactive_slice in &inactive_shape.slices {
                            for (j, _) in inactive_slice.filling.chars().enumerate() {
                                let inactive_cell_x_pos = inactive_slice.x + j as i32;
                                if cell_x_pos + 1 == inactive_cell_x_pos && slice.y == inactive_slice.y {
                                    return true
                                }
                            }
                        }
                    }
                }
            }
        },
        // move left
        "x-" => {
            // hit left wall
            for slice in active_shape.slices.iter() {
                if slice.x - 1 <= 0 {
                    return true
                }

                for (i, _) in slice.filling.chars().enumerate() {
                    let cell_x_pos = slice.x + i as i32;
                    for inactive_shape in inactive_shapes {
                        for inactive_slice in &inactive_shape.slices {
                            for (j, _) in inactive_slice.filling.chars().enumerate() {
                                let inactive_cell_x_pos = inactive_slice.x + j as i32;
                                if cell_x_pos - 1 == inactive_cell_x_pos && slice.y == inactive_slice.y {
                                    return true
                                }
                            }
                        }
                    }
                }
            }
        },
        // move down
        "y+" => {
            // hit bottom
            for slice in active_shape.slices.iter() {
                if slice.y + 1 >= RECT_LOWER_Y {
                    return true
                }

                for (i, _) in slice.filling.chars().enumerate() {
                    let cell_x_pos = slice.x + i as i32;
                    for inactive_shape in inactive_shapes {
                        for inactive_slice in &inactive_shape.slices {
                            for (j, _) in inactive_slice.filling.chars().enumerate() {
                                let inactive_cell_x_pos = inactive_slice.x + j as i32;
                                if cell_x_pos == inactive_cell_x_pos && slice.y + 1 == inactive_slice.y {
                                    return true
                                }
                            }
                        }
                    }
                }
            }

            
        },
        _ => panic!("unknown direction change")
    }

    false
}

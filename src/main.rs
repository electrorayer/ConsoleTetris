use core::time;
use std::{time::{Instant, Duration}, sync::RwLockWriteGuard, ops::Index, cell, process::exit, thread};

use console_engine::{screen::Screen, pixel, Color, ConsoleEngine, KeyCode};
use rand::{Rng, rngs::ThreadRng, seq::SliceRandom};


const WIDTH: i32 = 50;
const HEIGHT: i32 = 50;
const RECT_LOWER_X: i32 = 31;
const RECT_LOWER_Y: i32 = 30;


struct Shape {
    slices: Vec<Slice>,
    raw_slices: Vec<String>,
    color:  Color,
}

impl Shape {
    fn new(shape: &'static str) -> Shape {
        let colors = vec![Color::DarkGrey, Color::Red, Color::DarkRed, Color::Green, Color::DarkGreen, Color::Yellow, Color::DarkYellow, Color::Blue, Color::DarkBlue, Color::Magenta, Color::DarkMagenta, Color::Cyan, Color::DarkCyan];
        
        let default_x = 3;
        let default_y = 3;
        let mut slices: Vec<Slice> = vec![];
        let mut raw_slices: Vec<String> = vec![];
        for (y_offset, slice) in shape.split('\n').enumerate() {
            // calculate x axis offset for #
            let x_offset = (slice.chars().position(|c| c == '#').unwrap()) as i32;

            slices.push(Slice::new(default_x+x_offset, default_y+(y_offset as i32), slice.trim().to_string()));
            raw_slices.push(String::from(slice));
        }

        let rand_color = colors.choose(&mut rand::thread_rng()).unwrap().clone();
        Shape { slices, raw_slices, color: rand_color} 
    }

    fn bar() -> Shape {
        let l_shape = "########";
        Shape::new(l_shape)
    }

    fn cube() -> Shape {
        let l_shape = "####\n####";
        Shape::new(l_shape)
    }

    fn z_shape() -> Shape {
        let l_shape = "####  \n  ####";
        Shape::new(l_shape)
    }

    fn s_shape() -> Shape {
        let l_shape = "  ####\n####  ";
        Shape::new(l_shape)
    }

    fn l_shape() -> Shape {
        let l_shape = "######\n##    ";
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
            slice.x -= 2;
        }
    }

    fn go_right(&mut self) {
        for slice in &mut self.slices {
            slice.x += 2;
        }
    }

    fn rotate(&mut self) {
        // calculate closest and furthest x position for a shape
        // one "block" is 2 chars ##
        let mut closest_x = WIDTH;
        let mut furthest_x = 0;
        for slice in &self.slices {
            if slice.x < closest_x {
                closest_x = slice.x;
            }
            if slice.x + (slice.filling.chars().count() as i32) > furthest_x {
                furthest_x = slice.x + (slice.filling.chars().count() as i32);
            }
        }

        // TODO: When rotating, check for collision

        let new_number_of_slices = (furthest_x - closest_x) / 2;
        let mut new_slices: Vec<Slice> = vec![];
        let mut new_raw_slices: Vec<String> = vec![];

        for next_slice_offset in 0..new_number_of_slices {
            let mut new_starting_x_pos = closest_x;
            let new_starting_y_pos = self.slices[0].y + next_slice_offset;
            let mut new_raw_slice: String = String::new();
            let mut found_first_block = false;

            for raw_slice in &mut self.raw_slices.iter().rev() {
                // block can be "  " or "##"
                let block: String = raw_slice.chars().skip((next_slice_offset*2) as usize).take(2).collect();
                new_raw_slice.push_str(block.as_str());
                
                // starting position is where the first # appears
                if !found_first_block {
                    if new_raw_slice.contains("#") {
                        new_starting_x_pos += new_raw_slice.chars().position(|c| c == '#').unwrap() as i32;
                        found_first_block = true;
                    } 
                }
            }

            new_raw_slices.push(new_raw_slice.clone());
            new_slices.push(Slice::new(new_starting_x_pos, new_starting_y_pos, new_raw_slice.trim().to_string()));
        }

        // swap slices
        self.raw_slices = new_raw_slices;
        self.slices = new_slices;

        // if block goes out of bounds on the right side, go left
        let mut collision = false;
        let mut misplaced_blocks = 0;
        for slice in &self.slices {
            if slice.x + slice.filling.chars().count() as i32 - 1  >= RECT_LOWER_X {
                misplaced_blocks = (slice.x + slice.filling.chars().count() as i32 - RECT_LOWER_X) / 2;
                collision = true;
                break;
            }
        }

        for _ in 0..misplaced_blocks {
            self.go_left();
        }
        

    }    

}

struct Slice {
    x: i32, // start where first # is found
    y: i32, // different y pos per slice
    filling: String,
}

impl Slice {
    fn new(x: i32, y: i32, filling: String) -> Slice {
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
    //let mut active_shape = Shape::random_shape(&mut rng);
    let mut active_shape = Shape::t_shape();

    loop {
        engine.wait_frame();
        engine.clear_screen();
        
        // define box
        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg(' ', Color::White));

        // print inactive shapes
        for shape in &inactive_shapes {
            for slice in &shape.slices {
                engine.print_fbg(slice.x, slice.y, slice.filling.as_str(), shape.color, shape.color);
            }
        }

        // print active shape
        for slice in &active_shape.slices {
            engine.print_fbg(slice.x, slice.y, slice.filling.as_str(), active_shape.color, active_shape.color);
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

        // debug stuff
        // engine.print(10, 0, format!("{}", active_shape.rotate()).as_str());

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
        if engine.is_key_pressed(KeyCode::Char('z')) {
            // rotate
            active_shape.rotate();


            // check collision
            if has_collision(&inactive_shapes, &active_shape, "rotate") {
                panic!("Collided with another shape");
            }

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
    for slice in active_shape.slices.iter() {
        match direction_change {
            "x+" => {
                if slice.x + slice.filling.chars().count() as i32 >= RECT_LOWER_X {
                    return true
                }
            },
            "x-" => {
                if slice.x - 1 <= 0 {
                    return true
                }
            },
            "y+" => {
                if slice.y + 1 >= RECT_LOWER_Y {
                    return true
                }
            },
            "rotate" => {
                if (slice.x + slice.filling.chars().count() as i32 - 1 >= RECT_LOWER_X) || (slice.x <= 0) || (slice.y >= RECT_LOWER_Y) {
                    return true
                }
            },
            _ => panic!("unknown direction change"),
        }
        

        for (i, _) in slice.filling.chars().enumerate() {
            let cell_x_pos = slice.x + i as i32;
            for inactive_shape in inactive_shapes {
                for inactive_slice in &inactive_shape.slices {
                    for (j, _) in inactive_slice.filling.chars().enumerate() {
                        let inactive_cell_x_pos = inactive_slice.x + j as i32;
                        match direction_change {
                            "x+" => {
                                if cell_x_pos + 1 == inactive_cell_x_pos && slice.y == inactive_slice.y {
                                    return true
                                }
                            },
                            "x-" => {
                                if cell_x_pos - 1 == inactive_cell_x_pos && slice.y == inactive_slice.y {
                                    return true
                                }
                            },
                            "y+" => {
                                if cell_x_pos == inactive_cell_x_pos && slice.y + 1 == inactive_slice.y {
                                    return true
                                }
                            },
                            "rotate" => {
                                if cell_x_pos == inactive_cell_x_pos && slice.y == inactive_slice.y {
                                    // 2 cells can't have the same coordinates
                                    return true
                                }
                            },
                            _ => panic!("unknown direction change"),
                        }
                    }
                }
            }
        }
    }

    false
}

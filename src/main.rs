use core::time;
use std::{time::{Instant, Duration}, sync::RwLockWriteGuard};

use console_engine::{screen::Screen, pixel, Color, ConsoleEngine, KeyCode};


const WIDTH: i32 = 20;
const HEIGHT: i32 = 10;
const RECT_LOWER_X: i32 = WIDTH - 1;
const RECT_LOWER_Y: i32 = HEIGHT - 1;

fn main() {

    let mut engine = ConsoleEngine::init(WIDTH as u32, HEIGHT as u32, 80).unwrap();
    let mut line_y = 3;
    let mut start_x = 3;
    let mut stopwatch = Instant::now();
    let mut old_frame_count = 0;
    let mut elapsed_frames = 0;
    let bar = "      ";
    let vbar = "#\n#";

    loop {
        engine.wait_frame();
        engine.clear_screen();

        engine.rect(0, 1, RECT_LOWER_X, RECT_LOWER_Y, pixel::pxl_bg('€', Color::Black));
        engine.print_fbg(start_x, line_y, bar, Color::White, Color::AnsiValue(201));

        // debug coordinates
        engine.print(10, 0, format!("Y:{} X:{}", line_y, start_x).as_str());


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
            if line_y+1 < RECT_LOWER_Y {
                line_y += 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('z')) {
            if line_y-1 > 1 {
                line_y -= 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('q')) {
            if start_x-1 > 0 {
                start_x -= 1;
            }
        }

        else if engine.is_key_pressed(KeyCode::Char('d')) {
            if start_x + (bar.len() as i32) < RECT_LOWER_X {
                start_x += 1;
            }
        }


        engine.draw();
    }

    
}

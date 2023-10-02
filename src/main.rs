use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder};
use rand::Rng;

const WIDTH: usize = 1400;
const HEIGHT: usize = 850;
const OFFSCREEN_OFFSET: f32 = 1000.0; // Increase this value to increase the range of random x-coordinates
const GAP_HEIGHT: f32 = 200.0; // The height of the gap between the upper and lower tubes
const NUM_PIPES: usize = 5; // The number of pipes
const PIPE_DISTANCE: f32 = 500.0; // The minimum distance between each pair of pipes

struct Pipe {
    x: f32,
    gap_y: f32,
}

fn main() {
    let mut window = Window::new("Fbird", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut rng = rand::thread_rng(); // Create a random number generator

    // Initialize the pipes with random x-coordinates and gap positions
    let mut pipes: Vec<Pipe> = Vec::new();
        for i in 0..NUM_PIPES {
            let x = WIDTH as f32 + OFFSCREEN_OFFSET + i as f32 * PIPE_DISTANCE;
            let gap_y = rng.gen_range(GAP_HEIGHT..HEIGHT as f32 - GAP_HEIGHT);
            pipes.push(Pipe { x, gap_y });
        }

    loop { 
        for i in (0..pipes.len()).rev() {
            pipes[i].x -= 10.0; // Decrement x in each iteration to move the tubes to the left
        
            // If the tubes have reached the left edge of the window, reset x to a fixed x-coordinate off the right edge of the screen
            // and generate a new random y-coordinate for the gap
            if pipes[i].x + 100.0 < 0.0 {
                pipes[i].x = WIDTH as f32 + OFFSCREEN_OFFSET;
                pipes[i].gap_y = rng.gen_range(GAP_HEIGHT..HEIGHT as f32 - GAP_HEIGHT);
            }
            if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                // Convert the mouse position to integer coordinates
                let mouse_x = mouse_x as usize;
                let mouse_y = mouse_y as usize;
        
                // Check if the mouse is within the window bounds
                if mouse_x < WIDTH && mouse_y < HEIGHT {
                    // Get the color of the pixel at the mouse's current position
                    let pixel_color = dt.get_data()[(mouse_y * WIDTH + mouse_x) as usize];
        
                    // Extract the green component of the pixel color
                    let green = (pixel_color >> 8) & 0xff;
        
                    // Check if the mouse is within the bounds of any of the pipes
                    for pipe in &pipes {
                        if mouse_x >= pipe.x as usize && mouse_x <= (pipe.x + 100.0) as usize &&
                           (mouse_y <= (pipe.gap_y - GAP_HEIGHT / 2.0) as usize || mouse_y >= (pipe.gap_y + GAP_HEIGHT / 2.0) as usize) {
                            // If it is, and the green component is greater than a certain threshold, abort the process
                            if green > 230 {
                                std::process::abort();
                            }
                        }
                    }
                }
            }
        }
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        let mut pb = PathBuilder::new(); // Initialize pb

        for pipe in &pipes {
            // Draw the upper tube
            pb.rect(pipe.x, 0.0, 100., pipe.gap_y - GAP_HEIGHT / 2.0);
            // Draw the lower tube
            pb.rect(pipe.x, pipe.gap_y + GAP_HEIGHT / 2.0, 100., HEIGHT as f32 - pipe.gap_y - GAP_HEIGHT / 2.0);
        }

        let path = pb.finish();
        dt.fill(&path, &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)), &DrawOptions::new());

        window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    }
}
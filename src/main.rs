use minifb::{Window, WindowOptions};
use raqote::{DrawTarget, SolidSource, Source, DrawOptions, PathBuilder};
use rand::Rng;

const WIDTH: usize = 1400;
const HEIGHT: usize = 850;
const OFFSCREEN_OFFSET: f32 = 1000.0; // Increase this value to increase the range of random x-coordinates
const GAP_HEIGHT: f32 = 200.0; // The height of the gap between the upper and lower tubes
const NUM_PIPES: usize = 100; // The number of pipes
const PIPE_DISTANCE: f32 = 30.0; // The minimum distance between each pair of pipes

struct Pipe {
    x: f32,
    gap_y: f32,
}

fn main() {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions {
        ..WindowOptions::default()
    }).unwrap();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut rng = rand::thread_rng(); // Create a random number generator

    // Initialize the pipes with random x-coordinates and gap positions
    let mut pipes: Vec<Pipe> = Vec::new();
    let mut last_x = 0.0;
    for _ in 0..NUM_PIPES {
        let x = if last_x + PIPE_DISTANCE < WIDTH as f32 + OFFSCREEN_OFFSET {
            rng.gen_range(last_x + PIPE_DISTANCE..WIDTH as f32 + OFFSCREEN_OFFSET)
        } else {
            WIDTH as f32 + OFFSCREEN_OFFSET
        };
        let gap_y = rng.gen_range(GAP_HEIGHT..HEIGHT as f32 - GAP_HEIGHT);
        pipes.push(Pipe { x, gap_y });
        last_x = x;
    }

    let pipes_len = pipes.len() as f32; // Store the length of the pipes vector

    loop {
        for i in (0..pipes.len()).rev() {
            pipes[i].x -= 10.0; // Decrement x in each iteration to move the tubes to the left

            // If the tubes have reached the left edge of the window, reset x to a random x-coordinate off the right edge of the screen
            // and generate a new random y-coordinate for the gap
            if pipes[i].x + 100.0 < 0.0 {
                pipes[i].x = rng.gen_range(WIDTH as f32..WIDTH as f32 + OFFSCREEN_OFFSET) + pipes_len * PIPE_DISTANCE;
                pipes[i].gap_y = rng.gen_range(GAP_HEIGHT..HEIGHT as f32 - GAP_HEIGHT);
            }

            // Check if this pipe is touching any other pipe
            for j in 0..pipes.len() {
                if i != j && (pipes[i].x - pipes[j].x).abs() < PIPE_DISTANCE {
                    // If it is, remove this pipe
                    pipes.remove(i);
                    break;
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
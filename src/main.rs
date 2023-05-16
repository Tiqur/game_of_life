use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const DIM: usize = 512;
const CELL_SIZE: usize = 4;
const CELL_ROW_LEN: usize = DIM / CELL_SIZE;
const FPS: u64 = 15;

fn draw_world(
    buffer: &mut Vec<u32>,
    world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN],
    window: &mut Window,
    alive_color: u32,
    dead_color: u32,
) {
    // Iterate over cells in world and draw squares from pixels onto buffer
    for y in 0..CELL_ROW_LEN {
        for x in 0..CELL_ROW_LEN {
            let cell_start_index = y * CELL_SIZE * DIM + x * CELL_SIZE;
            let cell_color = if world[y][x] { dead_color } else { alive_color };

            // Draw square
            for x_cell in 0..CELL_SIZE {
                for y_cell in 0..CELL_SIZE {
                    buffer[cell_start_index + x_cell + y_cell * DIM] = cell_color;
                }
            }
        }
    }
    window.update_with_buffer(&buffer, DIM, DIM).unwrap();
}

fn get_live_neighbor_count(
    cell_pos: (usize, usize),
    world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN],
) -> u8 {
    let mut live_neighbor_count = 0;
    let x = cell_pos.0;
    let y = cell_pos.1;
    let x32 = x as i32;
    let y32 = y as i32;

    // Iterate over all neighbors ( except for center )
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            // Ensure array indexing is within bounds
            let new_x32 = x32.checked_add(j);
            let new_y32 = y32.checked_add(i);
            if let (Some(new_x), Some(new_y)) = (new_x32, new_y32) {
                let new_x_usize = new_x as usize;
                let new_y_usize = new_y as usize;

                if let Some(cell) = world.get(new_y_usize).and_then(|row| row.get(new_x_usize)) {
                    if *cell {
                        live_neighbor_count += 1;
                    }
                }
            }
        }
    }

    live_neighbor_count
}

fn get_updated_world(
    world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN],
) -> [[bool; CELL_ROW_LEN]; CELL_ROW_LEN] {
    // Create new world to return
    let mut next_world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN] =
        [[false; CELL_ROW_LEN]; CELL_ROW_LEN];

    // Iterate over world and calculate new frame
    for y in 0..CELL_ROW_LEN {
        for x in 0..CELL_ROW_LEN {
            let live_neighbor_count = get_live_neighbor_count((x, y), world);
            next_world[y][x] = match (world[y][x], live_neighbor_count) {
                (true, 2..=3) => true, // Live cell with 2 or 3 live neighbors survives
                (true, _) => false, // Live cell with fewer than 2 or more than 3 live neighbors dies
                (false, 3) => true, // Dead cell with exactly 3 live neighbors becomes alive
                _ => next_world[y][x], // For all other cases, the cell state remains the same
            };
        }
    }

    next_world
}

fn main() {
    let mut rng = rand::thread_rng();

    // Define window information
    let mut buffer: Vec<u32> = vec![0; DIM * DIM];
    let mut window =
        Window::new("Conway's Game Of Life", DIM, DIM, WindowOptions::default()).unwrap();

    // Init world
    let mut world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN] = [[false; CELL_ROW_LEN]; CELL_ROW_LEN];

    // Init colors
    let cell_alive_color = 255 << 16 | 255 << 8 | 255;
    let cell_dead_color = 0;
    let paused_cell_alive_color = 200 << 16 | 200 << 8 | 200;
    let paused_cell_dead_color = 128 << 16 | 128 << 8 | 128;

    // Set random
    for y in 0..CELL_ROW_LEN {
        for x in 0..CELL_ROW_LEN {
            let random = rng.gen();
            world[x][y] = random;
        }
    }

    // Time to sleep
    let frame_duration = std::time::Duration::from_secs_f64(1.0 / FPS as f64);

    // Game pause state
    let mut is_paused = false;

    // Graphics loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Pause game
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            is_paused = true;
        }

        // What the function says :P
        draw_world(
            &mut buffer,
            world,
            &mut window,
            if is_paused {
                paused_cell_alive_color
            } else {
                cell_alive_color
            },
            if is_paused {
                paused_cell_dead_color
            } else {
                cell_dead_color
            },
        );

        // Update world if not paused
        if !is_paused {
            world = get_updated_world(world);

            // Also don't need to worry about FPS when paused
            // If anything, it's better to have infinite FPS when accepting user input
            std::thread::sleep(frame_duration);
        }
    }
}

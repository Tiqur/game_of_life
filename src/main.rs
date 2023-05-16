use minifb::{Key, Window, WindowOptions};

const DIM: usize = 512;
const CELL_SIZE: usize = 32;
const CELL_ROW_LEN: usize = DIM / CELL_SIZE;
const FPS: u64 = 15;

fn draw_world(buffer: &mut Vec<u32>, world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN], window: &mut Window) {
    for y in 0..world.len() {
        for x in 0..CELL_ROW_LEN {
            let cell_start_index = y * CELL_SIZE*DIM+x*CELL_SIZE;
            let cell_color = if world[x][y] { 0 } else { 255 << 16 | 255 << 8 | 255 };
            for x_cell in 0..CELL_SIZE {
                for y_cell in 0..CELL_SIZE {
                    buffer[cell_start_index+x_cell+y_cell*DIM] = cell_color;
                }
            }
        }
    }
    window.update_with_buffer(&buffer, DIM, DIM)
        .unwrap();
}

fn get_live_neighbor_count(cell_pos: (usize, usize), world: &[[bool; CELL_ROW_LEN]; CELL_ROW_LEN]) -> u8 {
    let mut live_neighbor_count = 0;
    let x = cell_pos.0;
    let y = cell_pos.1;

    if x > 0 && world[x - 1][y] { live_neighbor_count += 1 };
    if x < CELL_ROW_LEN - 1 && world[x + 1][y] { live_neighbor_count += 1 };
    if y > 0 && world[x][y - 1] { live_neighbor_count += 1 };
    if y < CELL_ROW_LEN - 1 && world[x][y + 1] { live_neighbor_count += 1 };

    live_neighbor_count
}

fn update_world(world: &mut [[bool; CELL_ROW_LEN]; CELL_ROW_LEN]) {
    for x in 0..world.len() {
        for y in 0..CELL_ROW_LEN {
            let live_neighbor_count = get_live_neighbor_count((x, y), world);
            world[x][y] = match (world[x][y], live_neighbor_count) {
                (true, 2..=3) => true,      // Live cell with 2 or 3 live neighbors survives
                (true, _) => false,         // Live cell with fewer than 2 or more than 3 live neighbors dies
                (false, 3) => true,         // Dead cell with exactly 3 live neighbors becomes alive
                _ => world[x][y],           // For all other cases, the cell state remains the same
            };
        }
    }
}

fn main() {
    // Define window information
    let mut buffer: Vec<u32> = vec![0; DIM*DIM];
    let mut window = Window::new(
        "Conway's Game Of Life",
        DIM,
        DIM,
        WindowOptions::default()
    ).unwrap();

    // Init world
    let mut world: [[bool; CELL_ROW_LEN]; CELL_ROW_LEN] = [[false; CELL_ROW_LEN]; CELL_ROW_LEN];

    world[2][2] = true;
    world[2][3] = true;
    world[3][2] = true;
    world[3][3] = true;

    // Time to sleep
    let frame_duration = std::time::Duration::from_secs_f64(1.0 / FPS as f64);

    // Graphics loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(frame_duration);
        update_world(&mut world);
        draw_world(&mut buffer, world, &mut window);
    }
}

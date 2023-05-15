use minifb::{Key, Window, WindowOptions};

const DIM: usize = 512;
const CELL_SIZE: usize = 128;
const CELL_ROW_LEN: usize = DIM / CELL_SIZE;

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

    world[0][0] = true;

    // Graphics loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_world(&mut buffer, world, &mut window);
    }
}

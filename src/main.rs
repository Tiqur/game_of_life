use minifb::{Key, Window, WindowOptions};

const DIM: usize = 512;
const CELL_SIZE: usize = 128;
const CELL_ROW_COUNT: usize = DIM / CELL_SIZE;

fn main() {
    // Define window information
    let mut buffer: Vec<u32> = vec![0; DIM*DIM];
    let mut window = Window::new(
        "Conway's Game Of Life",
        DIM,
        DIM,
        WindowOptions::default()
    ).unwrap();

    let mut world: [[bool; CELL_ROW_COUNT]; CELL_ROW_COUNT] = [[false; CELL_ROW_COUNT]; CELL_ROW_COUNT];

    world[0][0] = true;

    // Graphics loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        for y in 0..world.len() {
            for x in 0..world[y].len() {
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
}

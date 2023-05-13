use minifb::{Key, Window, WindowOptions};

const DIM: usize = 512;

fn main() {
    // Define window information
    let mut buffer: Vec<u32> = vec![0; DIM*DIM];
    let mut window = Window::new(
        "Conway's Game Of Life",
        DIM,
        DIM,
        WindowOptions::default()
    ).unwrap();

    let mut world: [bool; DIM*DIM] = [true; DIM*DIM];

    // Define cell size
    let cell_size = 8;

    // Graphics loop
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let mut cell_index = 0;
        for cell_color in buffer.iter_mut() {
            *cell_color = if world[cell_index] == true { 255 << 16 | 255 << 8 | 255 } else { 0 };
            cell_index+=1;
        }

        window.update_with_buffer(&buffer, DIM, DIM)
            .unwrap();
    }
}

use minifb::{Key, Window, WindowOptions};

const DIM: usize = 640;

fn main() {
    let mut buffer: Vec<u32> = vec![0; DIM*DIM];
    let mut window = Window::new(
        "Conway's Game Of Life",
        DIM,
        DIM,
        WindowOptions::default()
    ).unwrap();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        window.update_with_buffer(&buffer, DIM, DIM)
            .unwrap();
    }
}

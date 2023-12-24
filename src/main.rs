extern crate glfw;
extern crate gl;
extern crate image;

use glfw::{Action, Context, Key};
use image::{ImageBuffer, Rgb};

fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        img.put_pixel(x as u32, y as u32, color);

        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x += sx; }
        if e2 <= dx { err += dx; y += sy; }
    }
}

fn draw_triangle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, color: Rgb<u8>) {
    draw_line(img, x0, y0, x1, y1, color);
    draw_line(img, x1, y1, x2, y2, color);
    draw_line(img, x2, y2, x0, y0, color);
}

fn run_image() {
    let imgx = 800;
    let imgy = 800;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    let red = Rgb([255, 0, 0]);
    draw_triangle(&mut imgbuf, 300, 300, 500, 300, 400, 500, red);

    imgbuf.save("triangle.png").unwrap();
    println!("Triangle image saved as 'triangle.png'");
}

fn main() {
    run_image();
    // let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // let (mut window, events) = glfw.create_window(800, 600, "Real-Time Renderer", glfw::WindowMode::Windowed)
    //     .expect("Failed to create GLFW window.");

    // window.set_key_polling(true);
    // window.make_current();

    // gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // while !window.should_close() {
    //     process_events(&mut window, &events);

    //     unsafe {
    //         gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    //         gl::Clear(gl::COLOR_BUFFER_BIT);
    //     }

    //     // TODO: Rendering code goes here

    //     window.swap_buffers();
    //     glfw.poll_events();
    // }
}

fn process_events(window: &mut glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITER: u32 = 100;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Mandelbrot {

}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mandelbrot = Mandelbrot::new();

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            mandelbrot.draw(pixels.frame_mut());
            if let Err(_) = pixels.render() {
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(_) = pixels.resize_surface(size.width, size.height) {
                    elwt.exit();
                    return;
                }
            }

            // Request a redraw
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

impl Mandelbrot {
    /// Create a new mandelbrot instance.
    fn new() -> Self {
        Self {

        }
    }

    fn mandelbrot(&self, x: u32, y: u32) -> u32 {
        // Map pixel coordinates to Mandelbrot set coordinates
        let x_min = -2.0;
        let x_max = 0.8;
        let y_min = -1.4;
        let y_max = 1.4;

        let x_coord = x_min + (x_max - x_min) * (x as f32) / (WIDTH as f32);
        let y_coord = y_min + (y_max - y_min) * (y as f32) / (HEIGHT as f32);

        let c = num::Complex::new(x_coord, y_coord);
        let mut z = num::Complex::new(0.0, 0.0);

        for n in 0..MAX_ITER {
            if z.norm() > 2.0 {
                return n;
            }
            z = z * z + c;
        }

        return MAX_ITER;
    }

    /// Draw the Mandelbrot state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as u32;
            let y = (i / WIDTH as usize) as u32;

            let m = self.mandelbrot(x, y);

            let rgba: [u8; 4] = if m == MAX_ITER {
                // In the Mandelbrot set
                [0, 0, 0, 255]
            } else {
                // Not in the Mandelbrot set
                // point escaped, color based on how quickly
                // using a simple red-yellow gradient
                [std::cmp::min(255, m*255 / 50) as u8,
                 std::cmp::min(255, m*255 / 100) as u8, 
                 0, 
                 255]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
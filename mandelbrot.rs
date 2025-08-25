extern crate num;
extern crate sdl2;

use num::{complex::{Complex32 as Complex}};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl};

const WIDTH : u32 = 512;
const HEIGHT : u32 = 512;

struct ViewportState {
    center : Complex,
    radius : f32,
    iter: i32
}

fn render(canvas: & mut Canvas<Window>, sdl: &Sdl, state: & ViewportState) {
    let timer  = sdl.timer().unwrap();
    let ticks_pre = timer.ticks64();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let norm_x: f32 = ((x as i32) - (WIDTH/2) as i32) as f32 * (state.radius / (WIDTH as f32));
            let norm_y: f32 = ((y as i32) - (HEIGHT/2) as i32) as f32 * (state.radius / (HEIGHT as f32));

            let c = Complex::new(norm_x, norm_y) + state.center;
            let mut z = c.clone();

            for i in 0..state.iter {
                z = z*z + c;

                
                if z.norm_sqr() > 4.0f32 {
                    canvas.set_draw_color(Color::RGB(0, 0, (255.0f32*(((i+1) as f32) / ((state.iter) as f32))) as u8));
                    canvas.draw_point((x as i32, y as i32)).expect("Driver error.");
                    break;
                }
            }
        }
    }

    let ticks_post = timer.ticks64();
    let title = format!("Rust SDL2 Mandelbrot ({} FPS, i={})", 1000.0f32 / ((ticks_post - ticks_pre) as f32), state.iter);
    canvas.window_mut().set_title(&title).expect("Error setting window title");
    canvas.present();
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();

    let window = sdl_video.window("Rust SDL2 Mandelbrot", WIDTH, HEIGHT)
                    .position_centered()
                    .build()
                    .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut state : ViewportState = ViewportState {
        center: Complex::new(-0.5f32, 0.0f32),
        radius: 4.0f32,
        iter: 16
    };

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => 
                    state.center += (state.radius / 10.0f32) * Complex::new(-1.0f32,  0.0f32),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => 
                    state.center += (state.radius / 10.0f32) * Complex::new( 1.0f32,  0.0f32),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => 
                    state.center += (state.radius / 10.0f32) * Complex::new( 0.0f32, -1.0f32),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => 
                    state.center += (state.radius / 10.0f32) * Complex::new( 0.0f32,  1.0f32),

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => 
                    state.radius /= 2.0f32,
                Event::KeyDown { keycode: Some(Keycode::E), .. } => 
                    state.radius *= 2.0f32,

                Event::KeyDown { keycode: Some(Keycode::A), .. } => 
                    state.iter -= 1,
                Event::KeyDown { keycode: Some(Keycode::D), .. } => 
                    state.iter += 1,

                _ => {}
            }
        }

        render(& mut canvas, &sdl, &state);
    }
}

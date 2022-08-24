extern crate sdl2;

use pyo3::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub mod engine_api;
use engine_api::*;


#[pyfunction]
pub fn run_engine() -> PyResult<()> {
    let mut app = App::new("RVGN Prototype", 640, 480);

    let mut canvas = app.window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    'running: loop {
        for event in app.events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }

            canvas.set_draw_color(Color::RGB(32, 24, 255));
            canvas.clear();
            canvas.present();
        }
    }

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn sdl_embed(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_engine, m)?)?;
    Ok(())
}
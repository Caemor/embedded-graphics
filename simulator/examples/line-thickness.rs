extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use embedded_graphics::{
    egtext, fonts::Font6x8, pixelcolor::Rgb888, prelude::*, primitive_style, primitives::ThickLine,
    style::PrimitiveStyle, text_style,
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};
use sdl2::keyboard::Keycode;

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const FOREGROUND_COLOR: Rgb888 = Rgb888::RED;

fn draw(
    display: &mut SimulatorDisplay<Rgb888>,
    position: Point,
    stroke_width: u32,
    draw_extra: bool,
) -> Result<(), core::convert::Infallible> {
    display.clear(BACKGROUND_COLOR)?;

    egtext!(
        text = &format!("W: {}", stroke_width),
        top_left = Point::zero(),
        style = text_style!(font = Font6x8, text_color = Rgb888::MAGENTA)
    )
    .draw(display)?;

    ThickLine::new(
        Point::new(
            display.size().width as i32 / 2,
            display.size().height as i32 / 2,
        ),
        position,
        primitive_style!(
            stroke_width = stroke_width,
            stroke_color = FOREGROUND_COLOR,
            // For debugging - right side of line uses this colour
            fill_color = Rgb888::GREEN
        ),
        draw_extra,
    )
    .into_iter()
    .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let mut window = WindowBuilder::new(&display)
        .scale(4)
        .title("Click to move circle")
        .build();

    let mut position = Point::new(150, 120);
    let mut stroke_width = 5;
    let mut draw_extra = true;
    let mut mouse_down = false;

    draw(&mut display, position, stroke_width, draw_extra)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Up => stroke_width += 1,
                        Keycode::Down => stroke_width = (stroke_width as i32 - 1).max(0) as u32,
                        Keycode::Space => draw_extra = !draw_extra,
                        _ => (),
                    }

                    draw(&mut display, position, stroke_width, draw_extra)?;
                }
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;
                    position = point;

                    draw(&mut display, position, stroke_width, draw_extra)?;
                }
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        position = point;
                        draw(&mut display, position, stroke_width, draw_extra)?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
pub mod config;
pub use config::Config;

use crate::game_of_life;
use crate::window;
use game_of_life::GameOfLife;

use sdl2::pixels::Color;
use std::convert::TryFrom;

pub fn display_game(window_data: &mut window::WindowData, game: &GameOfLife) -> window::Result<()> {
    let cell_width = window_data.window_config.size.cell_width;
    let cell_height = window_data.window_config.size.cell_height;
    assert!(i32::try_from(cell_width).is_ok());
    assert!(i32::try_from(cell_height).is_ok());

    window_data.canvas.set_draw_color(Color::WHITE);
    for (row, x) in game.into_iter().zip(0..) {
        for (&cell, y) in row.iter().zip(0..) {
            if cell {
                let x_point = x * cell_width as i32;
                let y_point = y * cell_height as i32;
                let rect = sdl2::rect::Rect::new(x_point, y_point, cell_width, cell_height);
                window_data.canvas.fill_rect(rect)?;
            }
        }
    }
    Ok(())
}

pub fn get_cell_from_window_pos(
    window_size: &window::WindowSize,
    (x, y): (i32, i32),
) -> (i32, i32) {
    let get_div = |lhs: i32, rhs: u32| -> i32 {
        lhs / i32::try_from(rhs).expect("cell size cannot be represented by i32")
    };

    (
        get_div(x, window_size.cell_width),
        get_div(y, window_size.cell_height),
    )
}

pub fn add_input_cb_to_handler<'a>(
    callback_handler: &mut window::CallbackHandler<'a, window::WindowData>,
    game: &'a std::cell::RefCell<GameOfLife>,
    game_config: &'a std::cell::RefCell<Config>,
) {
    callback_handler.add_event_cb_data({
        use game_of_life::shapes;
        let mut still_shapes = shapes::still::all().into_iter().cycle();
        let mut oscilator_shapes = shapes::oscilators::all().into_iter().cycle();
        let mut ship_shapes = shapes::ships::all().into_iter().cycle();
        let mut curious_shapes = shapes::curious::all().into_iter().cycle();

        move |window_data, event| {
            use sdl2::keyboard::Keycode;
            if let sdl2::event::Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } = event
            {
                let mouse_pos = || {
                    get_cell_from_window_pos(&window_data.window_config.size, window_data.mouse_pos)
                };
                match keycode {
                    Keycode::Space | Keycode::P => {
                        let mut game_config = game_config.borrow_mut();
                        game_config.running = !game_config.running;
                    }
                    Keycode::X => {
                        let mut game = game.borrow_mut();
                        game.randomize();
                    }
                    Keycode::C => {
                        let mut game = game.borrow_mut();
                        game.clear();
                    }
                    Keycode::Q => {
                        let pos = mouse_pos();
                        let mut game = game.borrow_mut();
                        let _ = still_shapes
                            .next()
                            .and_then(|shape| shape.add(&mut game, pos));
                    }
                    Keycode::W => {
                        let pos = mouse_pos();
                        let mut game = game.borrow_mut();
                        let _ = oscilator_shapes
                            .next()
                            .and_then(|shape| shape.add(&mut game, pos));
                    }
                    Keycode::E => {
                        let pos = mouse_pos();
                        let mut game = game.borrow_mut();
                        let _ = ship_shapes
                            .next()
                            .and_then(|shape| shape.add(&mut game, pos));
                    }
                    Keycode::R => {
                        let pos = mouse_pos();
                        let mut game = game.borrow_mut();
                        let _ = curious_shapes
                            .next()
                            .and_then(|shape| shape.add(&mut game, pos));
                    }
                    _ => {}
                }
            }
            if let sdl2::event::Event::MouseButtonDown {
                mouse_btn: sdl2::mouse::MouseButton::Left,
                clicks: 1,
                x,
                y,
                ..
            } = *event
            {
                let mut game = game.borrow_mut();
                let (x_cell, y_cell) =
                    get_cell_from_window_pos(&window_data.window_config.size, (x, y));
                let _ = game.toggle(x_cell, y_cell);
            }
            Ok(())
        }
    });
}

pub fn add_step_update_cb_to_handler<'a>(
    callback_handler: &mut window::CallbackHandler<'a, window::WindowData>,
    game: &'a std::cell::RefCell<GameOfLife>,
    game_config: &'a std::cell::RefCell<Config>,
) {
    callback_handler.add_frame_cb_data({
        let mut iteration = 0;
        move |_| {
            let game_config = game_config.borrow();
            if !game_config.running {
                return Ok(());
            }
            iteration += 1;
            if iteration >= game_config.ticks_to_update {
                iteration = 0;

                let mut game = game.borrow_mut();
                game.step();
            }
            Ok(())
        }
    });
}

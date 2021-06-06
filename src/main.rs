use std::convert::TryInto;

mod game_of_life;
mod game_of_life_io;
mod window;

fn get_window_config() -> Result<window::WindowConfig, Box<dyn std::error::Error>> {
    Ok(window::WindowConfig {
        window_name: "Game of life".into(),
        size: window::WindowSize {
            cells_in_width: 100,
            cells_in_height: 75,
            cell_width: 8,
            cell_height: 8,
        },
        update_frame_duration: window::fps_to_duration(60)?,
        background_color: sdl2::pixels::Color::BLACK,
    })
}

fn get_game_config() -> game_of_life_io::Config {
    game_of_life_io::Config {
        running: true,
        ticks_to_update: 5,
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window_config = get_window_config()?;

    let game = game_of_life::GameOfLife::create_random(
        window_config.size.cells_in_width.try_into()?,
        window_config.size.cells_in_height.try_into()?,
    );
    let game = std::cell::RefCell::new(game);

    let game_config = std::cell::RefCell::new(get_game_config());
    let mut callback_handler = window::CallbackHandler::default();

    game_of_life_io::add_input_cb_to_handler(&mut callback_handler, &game, &game_config);
    game_of_life_io::add_step_update_cb_to_handler(&mut callback_handler, &game, &game_config);

    callback_handler.add_frame_cb_data(|window_data| {
        game_of_life_io::display_game(window_data, &game.borrow())
    });

    callback_handler.add_event_cb(|event| {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Err(window::Error::Quit);
            }
            _ => {}
        }
        Ok(())
    });

    match window::run(window_config, callback_handler) {
        Ok(()) | Err(window::Error::Quit) => Ok(()),
        Err(window::Error::Err(e)) => Err(e),
        Err(e) => Err(e.into()),
    }
}

use tinybit::events::{events, Event, KeyCode, KeyEvent, EventModel};
use tinybit::{
    term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
    WorldPos, WorldSize, Pixel, Color
};
use tinybit::widgets::{Text, Border};

struct Mob {
    pixel: Pixel,
    world_pos: WorldPos,
    target_x: f32,
}

fn main() {
    let (width, height) = term_size().expect("Can't get the term size? Can't play the game!");

    // Viewport
    let viewport_size = ScreenSize::new(width / 2, height / 2);
    let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);
    let mut status_viewport = Viewport::new(ScreenPos::zero(), ScreenSize::new(width, 3));

    // Camera
    let (width, height) = (width as f32, height as f32 - 1.0);
    let camera_pos = WorldPos::new(0.0, 0.0);
    let mut camera = Camera::from_viewport(camera_pos, &viewport).with_limit(5, 5, 5, 5);

    // Renderer
    let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
    let mut renderer = Renderer::new(stdout_renderer);

    // Player
    let mut player = ('@', camera_pos);
    let mut player_pixel = Pixel::new(player.0, camera.to_screen(player.1), Some(Color::Yellow), None);

    // Mob
    let mut mob = Mob {
        pixel: Pixel::new('&', camera.to_screen(player.1), Some(Color::Red), None),
        world_pos: WorldPos::new(camera_pos.x - 2.0, camera_pos.y - 6.0),
        target_x: camera_pos.x + 10.0,
    };

    // "Grass"
    let grass_positions = field_of_grass(&camera);
    let mut grass = grass_positions.iter().map(|pos| Pixel::new(
        'w',
        camera.to_screen(camera.position),
        Some(Color::Green),
        None
    )).collect::<Vec<_>>();

    for event in events(EventModel::Fps(20)) {
        match event {
            Event::Tick => {
                viewport.draw_pixels(&grass);
                viewport.draw_pixel(player_pixel);
                mob.pixel.pos = camera.to_screen(mob.world_pos);
                viewport.draw_pixel(mob.pixel);

                // Move the mob to the right
                mob.world_pos.x += 0.1;
                let camera_border = Border::new("╭─╮│╯─╰│", Some(Color::White), None);
                viewport.draw_widget(&camera_border, ScreenPos::zero());

                // Track the player
                camera.track(player.1);
                player_pixel.pos = camera.to_screen(player.1);
                grass_positions.iter().zip(&mut grass).for_each(|(world_pos, grass)| {
                    grass.pos = camera.to_screen(*world_pos);
                });

                // Status
                let status = Text::new(
                    format!("player x {} | y {}  camera x {} | y {}",
                        player.1.x,
                        player.1.y, 
                        camera.position.x,
                        camera.position.y,
                    ),
                    Some(Color::Red),
                    None,
                );

                // Status border
                let status_border = Border::new("╭─╮│╯─╰│", Some(Color::White), None);

                status_viewport.swap_buffers();
                status_viewport.draw_widget(&status_border, ScreenPos::zero());
                status_viewport.draw_widget(&status, ScreenPos::new(1, 1));
                let _ = renderer.render(&mut status_viewport);

                // Render and swap the buffers so there is no 
                // residual characters
                let _ = renderer.render(&mut viewport);
                viewport.swap_buffers();
            }
            Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                match kc {
                    KeyCode::Left => { player.1.x -= 1.0; }
                    KeyCode::Right => { player.1.x += 1.0; }
                    KeyCode::Up => { player.1.y -= 1.0; }
                    KeyCode::Down => { player.1.y += 1.0; }
                    _ => {}
                }
            }
            Event::Resize(_w, _h) => {}
        }
    }
}

fn field_of_grass<T>(camera: &Camera<T>) -> Vec<WorldPos> {
    let pos = camera.position;
    let field_size = 3;
    let x_range = pos.x as i32 - field_size..pos.x as i32 + field_size;
    let y_range = pos.y as i32 - field_size..pos.y as i32 + field_size;

    let mut pixels = Vec::new();

    for x in x_range {
        for y in y_range.clone() {
            // let pixel = Pixel::new(
            //     '%', 
            //     camera.to_screen(WorldPos::new(x as f32, y as f32)),
            //     Some(Color::Green),
            //     None
            // );
            let pos = WorldPos::new(x as f32, y as f32);
            pixels.push(pos);
        }
    }

    pixels
}

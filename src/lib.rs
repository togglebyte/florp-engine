// #![deny(missing_docs)]
//! Terminal game engine
//! ```
//! use tinybit::events::{events, Event, KeyCode, KeyEvent, EventModel};
//! use tinybit::{
//!     term_size, Camera, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport,
//!     WorldPos, WorldSize, Pixel
//! };
//!
//! fn main() {
//!     let (width, height) = term_size().expect("Can't get the term size? Can't play the game!");
//!
//!     // Viewport
//!     let viewport_size = ScreenSize::new(width / 2, height / 2);
//!     let mut viewport = Viewport::new(ScreenPos::new(0, 4), viewport_size);
//!
//!     // Camera
//!     let (width, height) = (width as i64, height as i64);
//!     let camera_size = WorldSize::new(width / 2, height / 2);
//!     let camera_pos = WorldPos::new(width, height);
//!     let mut camera = Camera::new(camera_pos, camera_size);
//!
//!     // Renderer
//!     let stdout_renderer = StdoutTarget::new().expect("Failed to enter raw mode");
//!     let mut renderer = Renderer::new(stdout_renderer);
//!
//!     // Player
//!     let mut player = ('@', camera_pos);
//!
//!     for event in events(EventModel::Fps(20)) {
//!         match event {
//!             Event::Tick => {
//!                 let pixel = Pixel::new(player.0, camera.to_screen(player.1), None, None);
//!                 viewport.draw_pixel(pixel);
//!                 let _ = renderer.render(&mut viewport);
//!                 viewport.swap_buffers();
//! #               break
//!             }
//!             Event::Key(KeyEvent { code: KeyCode::Esc, ..  }) => break,
//!             Event::Key(KeyEvent { code: kc, .. }) => {
//!                 match kc {
//!                     KeyCode::Left => { player.1.x -= 1; }
//!                     KeyCode::Right => { player.1.x += 1; }
//!                     KeyCode::Up => { player.1.y -= 1; }
//!                     KeyCode::Down => { player.1.y += 1; }
//!                     _ => {}
//!                 }
//!             }
//!             Event::Resize(w, h) => {}
//!         }
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};

mod pixelbuffer;
mod viewport;

pub mod camera;
pub mod events;
pub mod render;
pub mod widgets;

/// A character at a position, with a colour
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pixel {
    pub glyph: char,
    pub pos: ScreenPos,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Pixel {
    pub fn new(
        glyph: char,
        pos: ScreenPos,
        fg_color: Option<Color>,
        bg_color: Option<Color>,
    ) -> Self {
        Self { glyph, pos, fg_color, bg_color }
    }

    pub fn white(c: char, pos: ScreenPos) -> Self {
        Self::new(c, pos, None, None)
    }
}

// -----------------------------------------------------------------------------
//     - Reexports -
// -----------------------------------------------------------------------------
pub use camera::Camera;
pub use crossterm::style::{Color, Colored};
pub use crossterm::terminal::size as term_size;
pub use crossterm::ErrorKind as CrosstermError;
pub use pixelbuffer::PixelBuffer;
pub use render::{Renderer, StdoutTarget};
pub use viewport::Viewport;

// -----------------------------------------------------------------------------
//     - Euclid -
// -----------------------------------------------------------------------------
/// A position on screen, where 0,0 is the top left corner
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl ScreenPos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

/// A position in the world
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct WorldPos {
    pub x: i64,
    pub y: i64,
}

impl WorldPos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

/// A rect on screen
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ScreenRect {
    pub origin: ScreenPos,
    pub size: ScreenSize,
}

impl ScreenRect {
    pub fn new(origin: ScreenPos, size: ScreenSize) -> Self {
        Self { origin, size }
    }
}

/// A rect in the world
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct WorldRect {
    pub origin: WorldPos,
    pub size: WorldSize,
}

impl WorldRect {
    pub fn new(origin: WorldPos, size: WorldSize) -> Self {
        Self { origin, size }
    }

    pub fn min_x(&self) -> i64 {
        self.origin.x
    }

    pub fn min_y(&self) -> i64 {
        self.origin.y
    }

    pub fn max_x(&self) -> i64 {
        self.origin.x + self.size.width
    }

    pub fn max_y(&self) -> i64 {
        self.origin.y + self.size.height
    }
}

/// A size on screen
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
}

impl ScreenSize {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

/// A size in the world
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct WorldSize {
    pub width: i64,
    pub height: i64,
}

impl WorldSize {
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }
}

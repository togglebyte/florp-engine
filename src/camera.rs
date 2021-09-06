use crate::{ScreenPos, Viewport, WorldPos, WorldRect, WorldSize};

pub struct Limit {
    top: i64,
    right: i64,
    bottom: i64,
    left: i64,
}

pub struct NoLimit;

/// Camera
pub struct Camera<T> {
    /// Global position
    pub position: WorldPos,

    size: WorldSize,
    pub(crate) bounding_box: WorldRect,
    limit: T,
}

impl<T> Camera<T> {
    /// Resize the camera
    pub fn resize(&mut self, new_size: WorldSize) {
        self.size = new_size;
    }

    /// Convert a point to local space.
    pub fn to_screen(&self, pos: WorldPos) -> ScreenPos {
        let min_x = self.bounding_box.min_x();
        let min_y = self.bounding_box.min_y();

        ScreenPos::new((pos.x - min_x) as u16, (pos.y - min_y) as u16)
    }

    /// Move the camera to a new position in global space
    pub fn move_to(&mut self, new_pos: WorldPos) {
        if new_pos == self.position {
            return;
        }

        self.position = new_pos;

        // Bounding box
        self.bounding_box = WorldRect::new(
            WorldPos::new(
                self.position.x - (self.size.width / 2),
                self.position.y - (self.size.height / 2),
            ),
            self.size,
        );
    }

}

impl Camera<NoLimit> {
    /// Create a new camera at a specific world position, with a fixed size.
    pub fn from_viewport(position: WorldPos, viewport: &Viewport) -> Camera<NoLimit> {
        let size = WorldSize::new(viewport.size.width as i64, viewport.size.height as i64);
        Self::new(position, size)
    }

    /// Create a new camera
    pub fn new(position: WorldPos, size: WorldSize) -> Camera<NoLimit> {
        let bounding_box = WorldRect::new(
            WorldPos::new(position.x - size.width / 2, position.y - size.height / 2),
            size,
        );

        Self {
            position,
            size,
            bounding_box,
            limit: NoLimit,
        }
    }

    /// The limit is used for tracking. For more information see `tracking`.
    /// given a limit of 1, 1, 1, 1, `c` represents the centre:
    ///
    /// ```text
    /// [ ] [ ] [ ] [ ] [ ]
    /// [ ] [x] [x] [x] [ ]
    /// [ ] [x] [c] [x] [ ]
    /// [ ] [x] [x] [x] [ ]
    /// [ ] [ ] [ ] [ ] [ ]
    /// ```
    pub fn with_limit(self, top: u16, right: u16, bottom: u16, left: u16) -> Camera<Limit> {
        Camera {
            limit: Limit {
                top: top as i64,
                right: right as i64,
                bottom: bottom as i64,
                left: left as i64,
            },
            position: self.position,
            bounding_box: self.bounding_box,
            size: self.size,
        }
    }
}
impl Camera<Limit> {
    /// Move the camera if the target is outside of the camera's `limit` box
    pub fn track(&mut self, pos: WorldPos) {
        let x = if pos.x >= self.position.x + self.limit.left {
            pos.x - self.limit.left
        } else if pos.x <= self.position.x - self.limit.right {
            pos.x + self.limit.right
        } else {
            self.position.x
        };

        let y = if pos.y >= self.position.y + self.limit.top {
            pos.y - self.limit.top
        } else if pos.y <= self.position.y - self.limit.bottom {
            pos.y + self.limit.bottom
        } else {
            self.position.y
        };

        self.move_to(WorldPos::new(x, y));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn camera() -> Camera<NoLimit> {
        let pos = WorldPos::new(3, 3);
        let size = WorldSize::new(6, 6);
        Camera::new(pos, size)
    }

    #[test]
    fn world_to_screen_position() {
        let cam = camera();
        let screen_pos = cam.to_screen(cam.bounding_box.origin);
        assert_eq!(screen_pos, ScreenPos::zero());
    }

    #[test]
    fn move_camera() {
        let mut cam = camera();
        let dest = WorldPos::new(100, 100);
        cam.move_to(dest);
        assert_eq!(dest, cam.position);
    }

    #[test]
    fn track_point() {
        let mut cam = camera();
        cam.move_to(WorldPos::new(100, 100));
        let mut cam = cam.with_limit(2, 2, 2, 2);

        let cam_pos = cam.position;

        cam.track(WorldPos::new(102, 98));
        assert_eq!(cam_pos, cam.position);

        // Don't move
        cam.move_to(WorldPos::new(100, 100));
        cam.track(WorldPos::new(100, 100));
        assert_eq!(WorldPos::new(100, 100), cam.position);

        // Move left
        cam.move_to(WorldPos::new(100, 100));
        cam.track(WorldPos::new(97, 98));
        assert_eq!(WorldPos::new(99, 100), cam.position);

        // Move right
        cam.move_to(WorldPos::new(100, 100));
        cam.track(WorldPos::new(103, 100));
        assert_eq!(WorldPos::new(101, 100), cam.position);

        // Move up
        cam.move_to(WorldPos::new(100, 100));
        cam.track(WorldPos::new(100, 103));
        assert_eq!(WorldPos::new(100, 101), cam.position);

        // Move down
        cam.move_to(WorldPos::new(100, 100));
        cam.track(WorldPos::new(100, 97));
        assert_eq!(WorldPos::new(100, 99), cam.position);
    }
}

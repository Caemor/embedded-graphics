//! TODO: Docs

use super::ThickLine;
use crate::geometry::Point;

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Side {
    /// TODO: Docs
    Left,
    /// TODO: Docs
    Right,
}

// /// TODO: Docs
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
// pub struct PerpLine {
//     start: Point,
//     end: Point,
//     width: i32,
// }

// impl PerpLine {
//     /// TODO: Docs
//     pub fn new(start: Point, end: Point, width: i32) -> Self {
//         Self { start, end, width }
//     }
// }

// impl IntoIterator for PerpLine {
//     type Item = Point;
//     type IntoIter = PerpLineIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         PerpLineIterator::new(&self, self.width)
//     }
// }

/// TODO: Docs
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct PerpLineIterator {
    error: i32,
    point: Point,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    width: u32,
    count: u32,
    side: Side,
    tk: i32,
    dx: i32,
    dy: i32,
    direction: Point,
}

impl PerpLineIterator {
    /// TODO: Docs
    pub fn new(
        start: Point,
        dx: i32,
        dy: i32,
        side: Side,
        width: u32,
        error: i32,
        winit: i32,
        direction: Point,
    ) -> Self {
        Self {
            error,
            direction,
            dx,
            dy,
            point: start,
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            width,
            count: 0,
            side,
            tk: match side {
                Side::Right => dx + dy - winit,
                Side::Left => dx + dy + winit,
            },
        }
    }
}

impl Iterator for PerpLineIterator {
    type Item = Point;

    // Octant 1 only
    fn next(&mut self) -> Option<Self::Item> {
        if self.tk > self.width as i32 {
            None
        } else {
            let point = self.point;

            match self.side {
                Side::Right => {
                    self.count += 1;

                    if self.error > self.threshold {
                        self.point -= Point::new(self.direction.x, 0);

                        self.error += self.e_diag;

                        self.tk += 2 * self.dy;
                    }

                    self.error += self.e_square;
                    self.point += Point::new(0, self.direction.y);

                    self.tk += 2 * self.dx;
                }
                Side::Left => {
                    self.count += 1;

                    if self.error < -self.threshold {
                        self.point += Point::new(self.direction.x, 0);

                        self.error -= self.e_diag;

                        self.tk += 2 * self.dy;
                    }

                    self.error -= self.e_square;
                    self.point -= Point::new(0, self.direction.y);

                    self.tk += 2 * self.dx;
                }
            }

            Some(point)
        }
    }
}

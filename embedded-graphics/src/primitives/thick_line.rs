//! TODO: Docs

use crate::drawable::Pixel;
use crate::geometry::Point;
use crate::pixelcolor::PixelColor;
use crate::primitives::perp_line::{PerpLineIterator, Side};
use crate::style::PrimitiveStyle;

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ThickLine<C: PixelColor> {
    start: Point,
    end: Point,
    style: PrimitiveStyle<C>,
    draw_extra: bool,
}

impl<C> ThickLine<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(start: Point, end: Point, style: PrimitiveStyle<C>, draw_extra: bool) -> Self {
        Self {
            start,
            end,
            style,
            draw_extra,
        }
    }
}

impl<C> IntoIterator for ThickLine<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = ThickLineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        ThickLineIterator::new(&self, self.style)
    }
}

/// TODO: Docs
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThickLineIterator<C: PixelColor> {
    error: i32,
    x: i32,
    y: i32,
    threshold: i32,
    e_diag: i32,
    e_square: i32,
    dx: i32,
    dy: i32,
    length: i32,
    style: PrimitiveStyle<C>,
    line: ThickLine<C>,
    count: i32,
    perp_left: PerpLineIterator,
    perp_right: PerpLineIterator,
    extra_perp_left: PerpLineIterator,
    extra_perp_right: PerpLineIterator,
    side_thickness: u32,
    p_error: i32,
    draw_extra: bool,
}

impl<C> ThickLineIterator<C>
where
    C: PixelColor,
{
    /// TODO: Docs
    pub fn new(line: &ThickLine<C>, style: PrimitiveStyle<C>) -> Self {
        use integer_sqrt::IntegerSquareRoot;

        let dx = line.end.x - line.start.x;
        let dy = line.end.y - line.start.y;

        // let side_thickness = style.stroke_width_i32() / 2;
        let side_thickness =
            2 * line.style.stroke_width * (dx.pow(2) as u32 + dy.pow(2) as u32).integer_sqrt();

        let error = 0;
        let p_error = 0;

        Self {
            error,
            x: line.start.x,
            y: line.start.y,
            dx,
            dy,
            line: line.clone(),
            threshold: dx - 2 * dy,
            e_diag: -2 * dx,
            e_square: 2 * dy,
            length: dx + 1,
            style,
            count: 0,
            draw_extra: line.draw_extra,
            perp_left: PerpLineIterator::new(
                line.start,
                dx,
                dy,
                Side::Left,
                side_thickness,
                p_error,
                error,
            ),
            perp_right: PerpLineIterator::new(
                line.start,
                dx,
                dy,
                Side::Right,
                side_thickness,
                p_error,
                error,
            ),
            extra_perp_left: PerpLineIterator::new(line.start, 1, 1, Side::Left, 0, p_error, 0),
            extra_perp_right: PerpLineIterator::new(line.start, 1, 1, Side::Right, 0, p_error, 0),
            side_thickness,
            p_error,
        }
    }
}

impl<C> Iterator for ThickLineIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    // Octant 1 only
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > self.length {
            None
        } else if let Some(point) = self.extra_perp_left.next() {
            Some(Pixel(point, self.style.fill_color.unwrap()))
        } else if let Some(point) = self.extra_perp_right.next() {
            Some(Pixel(point, self.style.stroke_color.unwrap()))
        } else if let Some(point) = self.perp_left.next() {
            Some(Pixel(point, self.style.stroke_color.unwrap()))
        } else if let Some(point) = self.perp_right.next() {
            Some(Pixel(point, self.style.fill_color.unwrap()))
        } else {
            self.count += 1;

            let mut extra = false;

            if self.error > self.threshold {
                self.y += 1;

                self.error += self.e_diag;

                if self.p_error > self.threshold {
                    self.p_error += self.e_diag;

                    if self.draw_extra {
                        self.extra_perp_left = PerpLineIterator::new(
                            Point::new(self.x, self.y),
                            self.dx,
                            self.dy,
                            Side::Left,
                            self.side_thickness,
                            self.p_error + self.e_square,
                            self.error,
                        );

                        self.extra_perp_right = PerpLineIterator::new(
                            Point::new(self.x, self.y),
                            self.dx,
                            self.dy,
                            Side::Right,
                            self.side_thickness,
                            self.p_error + self.e_square,
                            self.error,
                        );

                        extra = true;
                    }
                }

                self.p_error += self.e_square;
            }

            self.error += self.e_square;

            self.x += 1;

            self.perp_left = PerpLineIterator::new(
                Point::new(self.x, self.y),
                self.dx,
                self.dy,
                Side::Left,
                self.side_thickness,
                self.p_error,
                self.error,
            )
            .into_iter();

            self.perp_right = PerpLineIterator::new(
                Point::new(self.x, self.y),
                self.dx,
                self.dy,
                Side::Right,
                self.side_thickness,
                self.p_error,
                self.error,
            )
            .into_iter();

            if extra {
                self.extra_perp_left
                    .next()
                    .map(|point| Pixel(point, self.style.fill_color.unwrap()))
            } else {
                self.perp_left
                    .next()
                    .map(|point| Pixel(point, self.style.stroke_color.unwrap()))
            }
        }
    }
}
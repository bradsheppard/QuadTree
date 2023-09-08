use std::cmp;
use crate::{point::Point, rectangle::Rectangle};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: i64
}

impl Circle {
    pub fn intersects(&self, rectangle: &Rectangle) -> bool {
        let x = rectangle.top_left.x;
        let y = rectangle.top_left.y;

        let w = rectangle.bottom_right.x - rectangle.top_left.x;
        let h = rectangle.bottom_right.y - rectangle.top_left.y;

        let dx = self.center.x - cmp::max(x, cmp::min(self.center.x, x + w));
        let dy = self.center.y - cmp::max(y, cmp::min(y, self.center.y + h));

        return (dx * dx + dy * dy) <= self.radius * self.radius;
    }

    pub fn contains(&self, point: &Point) -> bool {
        return self.distance(&self.center, point) <= self.radius.pow(2);
    }

    fn distance(&self, point1: &Point, point2: &Point) -> i64 {
        return (point1.x - point2.x).pow(2) + (point1.y - point2.y).pow(2);
    }
}

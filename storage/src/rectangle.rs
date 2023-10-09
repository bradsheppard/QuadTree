use crate::point::Point;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point
}

impl Rectangle {
    pub fn new() -> Rectangle {
        return Rectangle { 
            top_left: Point { 
                x: 0.0 ,
                y: 0.0 
            }, 
            bottom_right: Point { 
                x: 0.0,
                y: 0.0 
            }
        }
    }

    pub fn from(top_left: &Point, bottom_right: &Point) -> Rectangle {
        return Rectangle { 
            top_left: *top_left,
            bottom_right: *bottom_right
        }
    }
}


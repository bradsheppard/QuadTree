#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Default, Debug)]
pub struct Quad {
    top_left: Point,
    bottom_right: Point,

    points: Vec<Point>,
    capacity: usize,
    is_child: bool,

    top_left_quad: Option<Box<Quad>>,
    top_right_quad: Option<Box<Quad>>,
    bottom_left_quad: Option<Box<Quad>>,
    bottom_right_quad: Option<Box<Quad>>
}

impl Quad {
    pub fn new() -> Quad {
        return Quad {
            top_left: Point{
                x: 0,
                y: 0
            },
            bottom_right: Point{
                x: 0,
                y: 0
            },

            points: vec![],
            capacity: 1,
            is_child: true,

            top_left_quad: None,
            top_right_quad: None,
            bottom_left_quad: None,
            bottom_right_quad: None
        }
    }

    pub fn from(top_left: Point, bottom_right: Point, capacity: usize) -> Quad {
        return Quad {
            top_left,
            bottom_right,

            points: vec![],
            capacity,
            is_child: true,

            top_left_quad: None,
            top_right_quad: None,
            bottom_left_quad: None,
            bottom_right_quad: None
        }
    }

    fn subdivide(&mut self) {
        let top_left_corner_1 = Point{
            x: self.top_left.x,
            y: self.top_left.y
        };

        let top_left_corner_2 = Point{
            x: (self.top_left.x + self.bottom_right.x) / 2,
            y: (self.top_left.y + self.bottom_right.y) / 2
        };

        let new_quad = Quad::from(top_left_corner_1, top_left_corner_2, self.capacity);
        self.top_left_quad = Some(Box::new(new_quad));

        let bottom_left_corner_1 = Point{
            x: self.top_left.x,
            y: (self.top_left.y + self.bottom_right.y) / 2
        };

        let bottom_left_corner_2 = Point{
            x: (self.top_left.x + self.bottom_right.x) / 2,
            y: self.bottom_right.y
        };

        let new_quad = Quad::from(bottom_left_corner_1, bottom_left_corner_2, self.capacity);
        self.bottom_left_quad = Some(Box::new(new_quad));

        let top_right_corner_1 = Point{
            x: (self.top_left.x + self.bottom_right.x) / 2,
            y: self.top_left.y
        };

        let top_right_corner_2 = Point{
            x: self.bottom_right.x,
            y: (self.top_left.y + self.bottom_right.y) / 2
        };

        let new_quad = Quad::from(top_right_corner_1, top_right_corner_2, self.capacity);
        self.top_right_quad = Some(Box::new(new_quad));

        let bottom_right_corner_1 = Point{
            x: (self.top_left.x + self.bottom_right.x) / 2,
            y: (self.top_left.y + self.bottom_right.y) / 2
        };

        let bottom_right_corner_2 = Point{
            x: self.bottom_right.x,
            y: self.bottom_right.y
        };

        let new_quad = Quad::from(bottom_right_corner_1, bottom_right_corner_2, self.capacity);
        self.bottom_right_quad = Some(Box::new(new_quad));
    }

    pub fn insert(&mut self, point: &Point) {
        if !self.check_boundary(&point) {
            return
        }

        if self.points.len() < self.capacity {
            self.points.push(point.to_owned());
        }
        else {
            if self.is_child {
                self.subdivide();

                for existing_point in &self.points {
                    self.top_left_quad.as_mut().unwrap().insert(existing_point);
                    self.bottom_left_quad.as_mut().unwrap().insert(existing_point);
                    self.top_right_quad.as_mut().unwrap().insert(existing_point);
                    self.bottom_right_quad.as_mut().unwrap().insert(existing_point);
                }

                self.points.clear();
                self.is_child = false
            }

            self.top_left_quad.as_mut().unwrap().insert(point);
            self.bottom_left_quad.as_mut().unwrap().insert(point);
            self.top_right_quad.as_mut().unwrap().insert(point);
            self.bottom_right_quad.as_mut().unwrap().insert(point);
        }
    }

    pub fn search(&self, point: &Point) -> bool {
        if !self.check_boundary(point) {
            return false
        }

        if self.is_child && self.points.contains(point) {
            return true
        }

        if (self.top_left.x + self.bottom_right.x) / 2 >= point.x {
            if (self.top_left.y + self.bottom_right.y) / 2 >= point.y {
                if self.top_left_quad.is_none() {
                    return false
                }

                let quad = self.top_left_quad.as_ref().unwrap();
                return quad.search(point)
            }
            else {
                if self.bottom_left_quad.is_none() {
                    return false
                }

                let quad = self.bottom_right_quad.as_ref().unwrap();
                return quad.search(point)
            }
        }
        else {
            if (self.top_left.y + self.bottom_right.y) / 2 >= point.y {
                if self.top_right_quad.is_none() {
                    return false;
                }

                let quad = self.top_right_quad.as_ref().unwrap();
                return quad.search(point)
            }
            else {
                if self.bottom_right_quad.is_none() {
                    return false;
                }

                let quad = self.bottom_right_quad.as_ref().unwrap();
                return quad.search(point)
            }
        }
    }

    fn check_boundary(&self, point: &Point) -> bool {
        return point.x >= self.top_left.x && point.x <= self.bottom_right.x && 
                point.y >= self.top_left.y && point.y <= self.bottom_right.y
    }
}

#[cfg(test)]
mod tests {
    use super::Quad;
    use super::Point;


    #[test]
    fn test_insert_and_get() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 4);

        let existant_point = Point{
            x: 5,
            y: 5
        };
        let nonexistant_point = Point{
            x: 7,
            y: 7
        };

        quad.insert(&existant_point);

        let existant_point_search = quad.search(&existant_point);
        let non_existant_point_search = quad.search(&nonexistant_point);

        assert_eq!(existant_point_search, true);
        assert_eq!(non_existant_point_search, false);
    }

}


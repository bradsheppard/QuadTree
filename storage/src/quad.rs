use crate::circle::Circle;
use crate::point::Point;
use crate::rectangle::Rectangle;

#[derive(Default, Debug)]
pub struct Quad {
    border: Rectangle,

    points: Vec<Point>,
    capacity: usize,
    is_leaf: bool,

    top_left_quad: Option<Box<Quad>>,
    top_right_quad: Option<Box<Quad>>,
    bottom_left_quad: Option<Box<Quad>>,
    bottom_right_quad: Option<Box<Quad>>
}

impl Quad {
    pub fn new() -> Quad {
        return Quad {
            border: Rectangle::new(),

            points: vec![],
            capacity: 1,
            is_leaf: true,

            top_left_quad: None,
            top_right_quad: None,
            bottom_left_quad: None,
            bottom_right_quad: None
        }
    }

    pub fn from(top_left: Point, bottom_right: Point, capacity: usize) -> Quad {
        return Quad {
            border: Rectangle::from(&top_left, &bottom_right),

            points: vec![],
            capacity,
            is_leaf: true,

            top_left_quad: None,
            top_right_quad: None,
            bottom_left_quad: None,
            bottom_right_quad: None
        }
    }

    fn subdivide(&mut self) {
        let top_left_corner_1 = Point{
            x: self.border.top_left.x,
            y: self.border.top_left.y
        };

        let top_left_corner_2 = Point{
            x: (self.border.top_left.x + self.border.bottom_right.x) / 2,
            y: (self.border.top_left.y + self.border.bottom_right.y) / 2
        };

        let new_quad = Quad::from(top_left_corner_1, top_left_corner_2, self.capacity);
        self.top_left_quad = Some(Box::new(new_quad));

        let bottom_left_corner_1 = Point{
            x: self.border.top_left.x,
            y: (self.border.top_left.y + self.border.bottom_right.y) / 2
        };

        let bottom_left_corner_2 = Point{
            x: (self.border.top_left.x + self.border.bottom_right.x) / 2,
            y: self.border.bottom_right.y
        };

        let new_quad = Quad::from(bottom_left_corner_1, bottom_left_corner_2, self.capacity);
        self.bottom_left_quad = Some(Box::new(new_quad));

        let top_right_corner_1 = Point{
            x: (self.border.top_left.x + self.border.bottom_right.x) / 2,
            y: self.border.top_left.y
        };

        let top_right_corner_2 = Point{
            x: self.border.bottom_right.x,
            y: (self.border.top_left.y + self.border.bottom_right.y) / 2
        };

        let new_quad = Quad::from(top_right_corner_1, top_right_corner_2, self.capacity);
        self.top_right_quad = Some(Box::new(new_quad));

        let bottom_right_corner_1 = Point{
            x: (self.border.top_left.x + self.border.bottom_right.x) / 2,
            y: (self.border.top_left.y + self.border.bottom_right.y) / 2
        };

        let bottom_right_corner_2 = Point{
            x: self.border.bottom_right.x,
            y: self.border.bottom_right.y
        };

        let new_quad = Quad::from(bottom_right_corner_1, bottom_right_corner_2, self.capacity);
        self.bottom_right_quad = Some(Box::new(new_quad));
    }

    pub fn insert(&mut self, point: &Point) {
        if !self.check_boundary(&point) {
            return
        }

        if self.is_leaf {
            if self.points.len() < self.capacity {
                self.points.push(point.to_owned());
            }
            else {
                self.subdivide();

                for existing_point in &self.points {
                    self.top_left_quad.as_mut().unwrap().insert(existing_point);
                    self.bottom_left_quad.as_mut().unwrap().insert(existing_point);
                    self.top_right_quad.as_mut().unwrap().insert(existing_point);
                    self.bottom_right_quad.as_mut().unwrap().insert(existing_point);
                }

                self.points.clear();
                self.is_leaf = false;

                self.top_left_quad.as_mut().unwrap().insert(point);
                self.bottom_left_quad.as_mut().unwrap().insert(point);
                self.top_right_quad.as_mut().unwrap().insert(point);
                self.bottom_right_quad.as_mut().unwrap().insert(point);
            }
        }
        else {
            self.top_left_quad.as_mut().unwrap().insert(point);
            self.bottom_left_quad.as_mut().unwrap().insert(point);
            self.top_right_quad.as_mut().unwrap().insert(point);
            self.bottom_right_quad.as_mut().unwrap().insert(point);
        }
    }

    pub fn delete(&mut self, point: &Point) {
        if !self.check_boundary(&point) {
            return
        }

        if self.is_leaf {
            self.points.retain(|x| *x != *point);
        }
        else {
            self.top_left_quad.as_mut().unwrap().delete(point);
            self.bottom_left_quad.as_mut().unwrap().delete(point);
            self.top_right_quad.as_mut().unwrap().delete(point);
            self.bottom_right_quad.as_mut().unwrap().delete(point);
        }
    }

    pub fn find_within_range(&self, circle: &Circle) -> Vec<Point> {
        let mut points = vec![];

        self.find_within_range_helper(circle, &mut points);

        return points;
    }

    pub fn find_within_range_helper(&self, circle: &Circle, points: &mut Vec<Point>) {
        if circle.intersects(&self.border) {
            if !self.is_leaf {
                self.top_left_quad.as_ref().unwrap().find_within_range_helper(circle, points);
                self.top_right_quad.as_ref().unwrap().find_within_range_helper(circle, points);
                self.bottom_left_quad.as_ref().unwrap().find_within_range_helper(circle, points);
                self.bottom_right_quad.as_ref().unwrap().find_within_range_helper(circle, points);
            }
            else {
                let mut found_points = self.points
                    .iter()
                    .filter(|p| circle.contains(p))
                    .map(|p| p.clone())
                    .collect::<Vec<Point>>();

                points.append(&mut found_points);
            }
        }
    }

    pub fn search(&self, point: &Point) -> bool {
        if !self.check_boundary(point) {
            return false
        }

        if self.is_leaf && self.points.contains(point) {
            return true
        }

        if (self.border.top_left.x + self.border.bottom_right.x) / 2 >= point.x {
            if (self.border.top_left.y + self.border.bottom_right.y) / 2 >= point.y {
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
            if (self.border.top_left.y + self.border.bottom_right.y) / 2 >= point.y {
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
        return point.x >= self.border.top_left.x && point.x <= self.border.bottom_right.x && 
                point.y >= self.border.top_left.y && point.y <= self.border.bottom_right.y
    }
}

#[cfg(test)]
mod tests {
    use crate::circle::Circle;

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

    #[test]
    fn test_insert_and_remove() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 4);

        let point = Point{
            x: 5,
            y: 5
        };

        quad.insert(&point);
        quad.delete(&point);

        let point_search = quad.search(&point);

        assert_eq!(point_search, false);
    }

    #[test]
    fn test_insert_and_get_with_subdivide() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 1);

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

    #[test]
    fn test_insert_and_remove_with_subdivide() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 1);

        let point = Point{
            x: 5,
            y: 5
        };

        quad.insert(&point);
        quad.delete(&point);

        let point_search = quad.search(&point);

        assert_eq!(point_search, false);
    }

    #[test]
    fn test_find_within_valid_range() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 1);

        let point = Point{
            x: 5,
            y: 5
        };

        quad.insert(&point);

        let circle = Circle{
            center: Point{
                x: 0,
                y: 0
            },
            radius: 10
        };

        let points = quad.find_within_range(&circle);

        assert_eq!(points.len(), 1);
        assert_eq!(points[0], point);
    }

    #[test]
    fn test_find_within_invalid_range() {
        let mut quad = Quad::from(Point{x: 0, y: 0}, Point{x: 100, y: 100}, 1);

        let point = Point{
            x: 5,
            y: 5
        };

        quad.insert(&point);

        let circle = Circle{
            center: Point{
                x: 0,
                y: 0
            },
            radius: 1
        };

        let points = quad.find_within_range(&circle);

        assert_eq!(points.len(), 0);
    }
}


// rectangle.rs
//
// Define the rectangle structure and functions that will be useful with it.

pub struct Rectangle {
    pub upper_x: i32,
    pub upper_y: i32,
    pub lower_x: i32,
    pub lower_y: i32
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
        Rectangle{upper_x: x, upper_y: y, lower_x: x + width, lower_y: y + height}
    }

    /// Returns true if this rectangle overlaps with the other
    pub fn intersect(&self, other: &Rectangle) -> bool {
        self.upper_x <= other.lower_x && self.lower_x >= other.upper_x && self.upper_y <= other.lower_y && self.lower_y >= other.upper_y
    }

    /// Returns the x, y coordinates of the center of the rectangle
    pub fn center(&self) -> (i32, i32) {
        ((self.upper_x + self.lower_x) / 2, (self.upper_y + self.lower_y) / 2)
    }
}

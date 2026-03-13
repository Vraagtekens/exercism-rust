use std::{ops::Add};
pub trait Number: Copy + Default + Add<Output = Self> + PartialOrd {}

impl Number for u32 {}
impl Number for f32 {}

pub struct Triangle<T: Number> {
    sides: [T; 3]
}

impl<T: Number> Triangle<T> {
    fn new(sides: [T; 3]) -> Self {
        Self { sides }
    }

    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let triangle = Triangle::new(sides);
        
        if triangle.is_triangle() {
            Some(triangle)
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, _, _] = self.sides;
        self.sides.iter().all(|x| x.eq(&a))
    }

    pub fn is_scalene(&self) -> bool {
        let [a, b, c] = self.sides;
        a != b && b != c && a != c
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || b == c || a == c
    }

    pub fn is_triangle(&self) -> bool {
        let is_zero = self.sides.contains(&T::default());

        !is_zero && self.inequality_violation()
    }

    pub fn inequality_violation(&self) -> bool {
        let [a, b, c] = self.sides;
        
        a + b >= c && b + c >=  a && a + c >= b
    }
}

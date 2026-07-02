use crate::Shape::Circle;

enum Shape {
    Circle { radius: f64 },
    Square { border: f64 },
    Rectangle { width: f64, height: f64 },
}
impl Shape {
    // TODO: Implement the `radius` method using
    //  either an `if let` or a `let/else`.
    // 只有Circle有radis，其他两个没有，所以可以不用考虑
    pub fn radius(&self) -> f64 {
        if let Shape::Circle { radius } = self{
            *radius
        }else {
            panic!("this shape does not have radius")
        }
        }
    }

#[cfg(test)]
// 测试逻辑，是Cricle就返回radis，否则返回panic
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let _ = Shape::Circle { radius: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_square() {
        let _ = Shape::Square { border: 1.0 }.radius();
    }

    #[test]
    #[should_panic]
    fn test_rectangle() {
        let _ = Shape::Rectangle {
            width: 1.0,
            height: 2.0,
        }
        .radius();
    }
}

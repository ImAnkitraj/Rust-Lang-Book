struct Point<T> {
    x: T,
    y: T,
}

// x() is available for all Point<T> types
// This is a generic function that works for any type T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// y() is only available for Point<i32> types
// This is a specialization of the Point<T> struct
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

struct PointTK<T, K> {
    x: T,
    y: K,
}

impl<T, K> PointTK<T, K> {
    fn mixup<V, W>(self, other: PointTK<V, W>) -> PointTK<T, W> {
        PointTK {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}
fn main() {
    let p = Point { x: 5, y: 10 };
    p.x();
    // p.y(); // Gives error
    let p2 = Point { x: 1.0, y: 2.0 };
    p2.x();
    p2.y();
}

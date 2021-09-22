pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        ((self.x.powf(2f64)) + (self.y.powf(2f64))).sqrt()
    }
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
pub struct DPoint<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> DPoint<T, U> {
    pub fn mixup<V, W>(self, other: DPoint<V, W>) -> DPoint<T, W> {
        DPoint {
            x: self.x,
            y: other.y,
        }
    }

    pub fn mixFirst<V, W>(self, other: DPoint<V, W>) -> (DPoint<T, V>, DPoint<U, W>) {
        (
            DPoint {
                x: self.x,
                y: other.x,
            },
            DPoint {
                x: self.y,
                y: other.y,
            },
        )
    }
}

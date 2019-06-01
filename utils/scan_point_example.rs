#[derive(Copy, Clone)]
struct Point {
    t: isize,
    x: isize,
    y: isize,
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            t: self.t - other.t,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn new() -> Point {
        Point { t: 0, x: 0, y: 0 }
    }

    fn is_reachable_from(self, p: Point) -> bool {
        let difference = p - self;
        let time_interval = difference.t;
        let distance = difference.x + difference.y; // assuming positive

        time_interval.has_same_parity(distance)
    }
}

fn scan_point(cin: &mut CharacterInput) -> Point {
    Point {
        t: cin.scan_u() as isize,
        x: cin.scan_u() as isize,
        y: cin.scan_u() as isize,
    }
}

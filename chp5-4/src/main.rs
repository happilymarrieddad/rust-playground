// Generics

fn squarei32(x: i32) -> i32 {
    x * x
}

fn squaref32(x: f32) -> f32 {
    x * x
}

// allows Add/Multiple/Copy traits (might not be called traits)
// fn square<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy>(x: T) -> T {
//     x * x
// }

// More readable
fn square<T>(x: T) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    x * x
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn printing(&self) {
        println!(
            "The value of point coordinates are {:?}, {:?}",
            self.x, self.y
        );
    }
}

fn main() {
    println!("Some val: {}", squarei32(5));
    println!("Some val: {}", squaref32(5.2));

    println!("Some val: {}", square(9));
    println!("Some val: {}", square(9.4));

    let p1 = Point { x: 5, y: 5 };
    let p2 = Point { x: 1.0, y: 4 };
    let p3 = Point { x: 1, y: 4. };
    let p4 = Point { x: 1., y: 4. };

    p1.printing();
    p2.printing();
    p3.printing();
    p4.printing();
}

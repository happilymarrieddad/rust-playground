mod file_1;
mod file_2;
mod file_3;

struct Rectangle {
    length: i32,
    width: i32,
}

fn main() {
    let rec1 = Rectangle {
        length: 5,
        width: 7,
    };

    file_1::rect_area(&rec1.length, &rec1.width);

    let np = file_2::new_person();

    file_3::allowance();
}

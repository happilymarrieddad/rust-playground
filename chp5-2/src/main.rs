struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }
    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }
    fn country_info(&self) -> &str {
        &self.country
    }
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

trait Calculations {
    fn area(&self) {
        // This is the default behavior
        println!("I am not implemented for this type");
    }
    fn perimeter(&self);
}

impl Calculations for Circle {
    fn area(&self) {
        let area_of_circle = 3.14 * (self.radius * self.radius);
        println!("The area of the circle is {}", area_of_circle);
    }
    fn perimeter(&self) {
        let perimeter_of_circle = 2. * 3.14 * self.radius;
        println!("The perimeter of the circle is {}", perimeter_of_circle);
    }
}

impl Calculations for Rectangle {
    fn area(&self) {
        let area_of_rectangle = self.length * self.width;
        println!("The area of the rectangle is {}", area_of_rectangle);
    }
    fn perimeter(&self) {
        let perimeter_of_rectangle = (self.length + self.width) * 2.;
        println!(
            "The perimeter of the rectangle is {}",
            perimeter_of_rectangle
        );
    }
}

struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

// funcs within a trait can use each other
impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum: i32 = 0;

        for i in self.some_data.iter() {
            sum += *i;
        }

        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mu = self.mean();

        let mut sum_squared_diff = 0.;

        for i in self.some_data.iter() {
            sum_squared_diff += (*i as f32 - mu) * (*i as f32 - mu);
        }

        sum_squared_diff / self.some_data.len() as f32
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Nick Kotenberg"),
        citizenship: String::from("USA"),
        age: 39,
        gender: 'M',
        salary: 40_000,
    };

    let student1 = Student {
        name_std: String::from("John Doe"),
        age: 36,
        sex: 'M',
        country: String::from("USA"),
    };

    println!("Person 1: {:?}", person1.info());
    println!("Student 1: {:?}", student1.info());

    let c1 = Circle { radius: 3.2 };

    let r1 = Rectangle {
        length: 5.,
        width: 4.,
    };

    c1.area();
    c1.perimeter();

    r1.area();
    r1.perimeter();

    let my_data = Data {
        some_data: vec![5, 6, 9, 8, 7, 4, 8],
    };

    println!("Mean: {}", my_data.mean());
    println!("Variance: {}", my_data.variance());
}

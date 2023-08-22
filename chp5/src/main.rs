struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Self {
        Person {
            citizenship: String::new(),
            name: String::new(),
            age: 0,
            gender: 'N',
            salary: 0,
        }
    }

    fn compute_taxes(&self, additional_taxes: f32) -> f32 {
        // 3. is a way to force a float
        ((self.salary as f32 / 3.) * 0.5) + additional_taxes
    }
}

struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {
            return self.0;
        }
        self.1
    }
}

fn main() {
    let extra_taxes = 10.;

    let person1 = Person {
        name: String::from("Nick Kotenberg"),
        citizenship: String::from("USA"),
        age: 39,
        gender: 'M',
        salary: 40_000,
    };

    println!("1");
    println!("{}", person1.name);
    println!("{}", person1.citizenship);
    println!("{}", person1.age);
    println!("{}", person1.gender);
    println!("{}", person1.salary);
    println!("{}", person1.compute_taxes(extra_taxes));
    println!("{}", Person::compute_taxes(&person1, extra_taxes));

    let person2 = Person::new();

    println!("2");
    println!("{}", person2.name);
    println!("{}", person2.citizenship);
    println!("{}", person2.age);
    println!("{}", person2.gender);
    println!("{}", person2.salary);
    println!("{}", person2.compute_taxes(extra_taxes));
    println!("{}", Person::compute_taxes(&person2, extra_taxes));

    let person3: Person = Person {
        age: 50,
        name: String::from("John Doe"),
        ..person2
    };

    println!("3");
    println!("{}", person3.name);
    println!("{}", person3.citizenship);
    println!("{}", person3.age);
    println!("{}", person3.gender);
    println!("{}", person3.salary);
    println!("{}", person3.compute_taxes(extra_taxes));
    println!("{}", Person::compute_taxes(&person3, extra_taxes));

    // Tuple structures
    let some_nums = Numbers(3, 2);
    println!("{} {} {}", some_nums.0, some_nums.1, some_nums.greater());
}

enum Coveyance {
    Car = 10, // you can set the starting value else it will be 0
    Train,
    Air = 30, // force 30
}

impl Coveyance {
    fn travel_allowance(&self, miles: i32) -> f32 {
        let allowance = match self {
            Coveyance::Car => miles as f32 * 14. * 2., // 2 is because it's a round trip
            Coveyance::Train => miles as f32 * 18. * 2.,
            Coveyance::Air => miles as f32 * 30. * 2.,
        };

        allowance
    }
}

enum Coveyance2 {
    Car(i32),
}

impl Coveyance2 {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            Coveyance2::Car(miles) => *miles as f32 * 14. * 2., // 2 is because it's a round trip
        };

        allowance
    }
}

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let participant_1: Coveyance = Coveyance::Car;

    println!("Value of trip: {}", participant_1.travel_allowance(60));

    let part_2: Coveyance2 = Coveyance2::Car(60);

    println!("Value of trip: {}", part_2.travel_allowance());

    let some_val = vec![Value::Integer(12), Value::Float(9.)];
    println!(
        "Value of int is {:?} and float is {:?}",
        some_val[0], some_val[1]
    );

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The value of int: {:?}", num),
            Value::Float(num) => println!("The value of float: {:?}", num),
        }
    }
}

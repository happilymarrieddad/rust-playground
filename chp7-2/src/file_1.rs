// New crate - the file

// parent modules can not see children modules unless set pub
mod maths {
    // children modules can see parent modules by default
    pub mod basic_math {
        pub fn mult(num1: &i32, num2: &i32) -> i32 {
            let res = num1 * num2;
            printing(&res);
            res
        }

        fn printing(num: &i32) {
            println!("The result is {}", num);
            crate::file_1::some_fn() // able to access crate func using the keyword crate
        }
    }
}

pub fn rect_area(l: &i32, w: &i32) -> i32 {
    use maths::basic_math::mult;
    mult(l, w)
}

fn some_fn() {
    println!("Some fn printing")
}

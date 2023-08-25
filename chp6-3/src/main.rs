fn main() {
    fn min(x: f32, y: f32) -> f32 {
        if x < y {
            x
        } else {
            y
        }
    }

    fn max(x: f32, y: f32) -> f32 {
        if x > y {
            x
        } else {
            y
        }
    }

    let mut f: fn(f32, f32) -> f32 = min;

    println!("Min: {}", f(1., 2.));

    f = max;

    println!("Min: {}", f(1., 2.));

    let v = vec![1, 3, 4, 5];
    let mut iter = v.iter();

    println!("{:?}", iter);
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let a = vec![3, 5, 7, 3, 2, -1, 6];
    let mut check = a.iter().any(|&x| x > 0);
    println!("Value of check: {}", check);

    let mut check = a.iter().all(|&x| x > 0);
    println!("Value of check: {}", check);

    // .iter uses reference so double reference returns the actual value
    let mut check = a.iter().find(|&&x| x > 0);
    println!("Value of check: {:?}", check);

    let mut check = a.iter().position(|&x| x > 5);
    println!("Value of check: {:?}", check);

    // Starts on the other side of the vector
    let mut check = a.iter().rposition(|&x| x > 5);
    println!("Value of check: {:?}", check);

    let a = vec![2, 6, 3, 34, 56, 7, 74, -1];
    let filtered_values = a.iter().filter(|&x| *x > 5).collect::<Vec<&i32>>();
    println!("{:?}", filtered_values);

    // if you want the actual values - note ownership change here
    let a = vec![2, 6, 3, 34, 56, 7, 74, -1];
    let b = a.clone();
    let filtered_values = a.into_iter().filter(|&x| x > 5).collect::<Vec<i32>>();
    println!("{:?}", filtered_values);

    let mut mv = b.iter().map(|x| 2 * *x).collect::<Vec<i32>>();
    println!("{:?}", mv);
}

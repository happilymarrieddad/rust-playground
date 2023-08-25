use array_tool::vec::*;

fn main() {
    let v1 = vec![1, 1, 3, 5, 6, 7];
    let v2 = vec![1, 2, 3];

    let intersection = v1.intersect(v2.clone());
    println!("The intersection = {:?}", intersection);

    let union_set = v1.union(v2.clone());
    println!("The union = {:?}", union_set);

    println!("Vec 1 three times displayed = {:?}", v1.times(3));
}

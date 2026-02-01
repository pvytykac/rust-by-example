fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3];
    vector.push(4);
    println!("{:?}, capacity: {}", vector, vector.capacity());

    let mut vector: Vec<i32> = Vec::with_capacity(3);
    vector.push(1);
    println!(
        "{:?}, length: {}, capacity: {}",
        vector,
        vector.len(),
        vector.capacity()
    );

    let mut vector: Vec<i32> = (0..4).collect();
    println!("{:?}, capacity: {}", vector, vector.capacity());

    vector.insert(4, 4);
    println!("{} == {:?}", vector[0], vector.first());
    println!("{} == {:?}", vector[4], vector.last());
    println!("exists: {}", vector.get(666).is_some());

    vector.pop();
    println!("{:?}", vector);

    vector.remove(0);
    println!("{:?}", vector);

    vector.clear();
    println!("{:?}", vector);
}

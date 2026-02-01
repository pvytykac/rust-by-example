use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::from([("Alice", 32), ("Bob", 39)]);
    ages.insert("Charlie", 26);
    ages.entry("Dominic").or_insert(30);
    println!("{:?}", ages);

    ages.entry("Dominic").and_modify(|age| *age += 1);
    println!("{:?}", ages);

    if let Some(age) = ages.get("Dominic") {
        println!("Dominic's age is now {}", age);
    }

    ages.remove("Dominic");
    if let None = ages.get("Dominic") {
        eprintln!("We don't know Dominic's age!");
    }

    ages.keys().for_each(|name| println!("{}", name));
}

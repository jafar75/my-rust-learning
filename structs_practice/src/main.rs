
struct Student {
    name: String,
    id: u64,
    age: u8
}

fn main() {
    println!("Hello, world!");

    let jafar = Student {
        name: String::from("Jafar"),
        age: 26,
        id: 12345678
    };

    let user2 = Student {
        age: 23,
        ..jafar
    };
    println!("{}", user2.age);

    println!("{}", jafar.name);     // Error: name is String that "moved" to user2
}

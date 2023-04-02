fn main() {
    // shadowing
    let spaces = "jafar";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");

    // tuples
    let tup : (f32, i128, bool) = (32.3, 12432423, false);
    let f = tup.0;
    println!("{f}");

    let x = 12;

    let y = x + 11;
    println!("{y}");

    example_func(12.2, false);
    example_func(12.33, true);

    println!("{}", example_func(21.39, true));
}

fn example_func (var1: f32, var2: bool) -> f32 {
    println!("inside example fn!!!!");
    if var2 {
        println!("{var1}");
    }
    var1 + 2.44
}

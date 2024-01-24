fn ten() -> i32 {
    10
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {

    let x = plus_one(8);

    println!("The value of x plus one is {x}");

    // recuerda que '' y "" son diferentes. El primero 
    // para char y el segundo para strings.
    another_function(5, 'h'); 
}

fn another_function(x: i32, unit_label: char) {
    println!("The current measurement is: {x}{unit_label}");
}

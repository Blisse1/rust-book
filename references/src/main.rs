// fn main() {
//     let m1 = String::from("Hello");
//     let m2 = String::from("world");
//     greet(&m1, &m2); // note the ampersands
//     let s = format!("{} {}", m1, m2);
//     println!("{s}");
// }
fn main() {
let x = Box::new(0);
let y = Box::new(&x);

let z = ***y;

println!("{z}");
}

// fn greet(g1: &String, g2: &String) { // note the ampersands
//     println!("{} {}!", g1, g2);
// }

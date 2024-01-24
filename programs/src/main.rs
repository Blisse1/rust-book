// use std::io;

fn main() {
    // println!("Enter a Farenheit value");

    // let mut farenheit = String::new();

    // io::stdin()
    //     .read_line(&mut farenheit)
    //     .expect("Failed to read line");

    // let farenheit: i32 = farenheit.trim().parse().expect("You should type a numeric value");

    // let calc: i32 = (farenheit - 32) * 5/9;

    // println!("The temperature in Celsius for the Farenheit you typed ({farenheit}) is {calc}");
    christmas_carol();
}

fn christmas_carol(){
    // on the {} of chrismas
    // my true love sent to me:
    // A patridge in a pear tree
    //
    // ...
    // Two turtle doves
    // and a patridge in a pear tree
    //
    // ...
    // three french hens
    // two turtle doves
    // and a patridge in a pear tree

    let days = ["first", "second", "third", "fourth", "fifth",
    "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];

    let gifts = ["A patridge in a pear tree", "Two turtle doves",
    "and a patridge in a pear tree", "three french hens", "Four calling birds",
    "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking",
    "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    println!("On the {} of Christmas \nmy true love sent to me: \n{} \n", days[0], gifts[0]);

}

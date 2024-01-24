fn main() {
    // loop {
    //     println!("Hello, world!");
    //     break;
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");
    //

    // while_conditional();
    // loop_labels();
    // looping_collection_while();
    looping_collection_for();

}

fn while_conditional() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");

}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // Aquí evalua primero de adentro hacia afuera
    // quítele 1 a remaining si no cumple condición y añadale 1 a count
    // println!("End count = {count}");
    // 1. remaining = 10 y count = 0
    // 2. remaining = 9 y count = 1 aplica los cambios
    // 3. remaining = 9 y count = 2 rompe el loop de afuera cuando llega count
    // a 2. Y el loop de fuera tiene a count
}

fn looping_collection_while() {
   let a = [10, 20, 30, 40, 50]; 
   let mut index = 0;

   while index < 5 {
       println!("the value is {}", a[index]);

       index += 1;
   }
}

fn looping_collection_for(){
   let a = [10, 20, 30, 40, 50]; 

   for number in a {
       println!("the value is {number}");
   }
}

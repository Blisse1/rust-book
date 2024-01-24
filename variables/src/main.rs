fn main() {


    // Shadowing 
    let x: u8 = 5;

    println!("{x}");

    let x = 4; // nueva variable inmutable

    println!("{x}");

    // Let mut
    let mut y: u32 = 5;

    y += 5;

    y = y - 2; // variable sujeta a cambios porque es mut 

    println!("{y}");

    // La diferencia entre shadowing y let es que 
    // al shadowing se le puede hacer algunas modificaciones
    // y la variable como tal queda inmutable despues de eso
    // y queda como una nueva variable cuando se vuelve a usar let 
    //
    // En cambio con let mut uno define que a lo largo del programa
    // se pueden realizar y seguir realizando modificaciones

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {


   let user1 =  User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };



    // println!("Hello, world!");
    // // let se utiliza para inmutables variables por defecto
    // let x = 5;
    // let  y = 10;
    // let sum = x + y;
    // println!("The sum of {} and {} is {}", x, y, sum);

    // anda_function();

    // let suma = plus_one(5);
    // println!("5 plus one is {}", suma);

    // is_mayor();

    // loop {
    //     println!("This loop will run forever.");// para evitar un bucle infinito
    //     break;
    // }

    // array();

    // array_while();

    println!("{}", user1.username);

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // pasamos una referencia a s1

    println!("{s1} {s2},world")
}
// Función simple que imprime un mensaje
// fn anda_function() {
//     println!("This is another function.");
// }
// // otro ejemplo de funcion
// fn plus_one(num: i32) -> i32 {
//     num + 1
// }

// // uso de if

// fn is_mayor() {

//     let numero = 7;
//     if numero < 5 {
//         println!("El número es menor que 5");
//     } else {
//         println!("El número es mayor o igual a 5");
//     }

// }

// // uso for

// fn array() {

//     let array = [10,20,30,40,50];
  
//     for value in array  {
//         println!("the value is {value}");
    
//     }
// } 

// fn array_while() {
  
//     let array = [60,70,80,90,100];
//     let mut index = 0;

//     while index < 5 {

//         println!("the value from array_while is {}",array[index]);

//         index += 1;
//     }

// }

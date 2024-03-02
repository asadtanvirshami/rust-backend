
//***********LECTURE 05***********\\

// use std::io; //import package for input

// // Input function that takes input as int
// fn main(){
//     let mut input = String::new();

//     println!("Enter an integer:");
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let num: i32 = input.trim().parse().expect("Invalid input"); //here this line is using iu32 to recognize input as integer. 
//     println!("{}",num + 3);

// }

//***********LECTURE 06***********\\

// fn main (){

    //******************DEFINE BITS******************//
    
    // u means unsigned integer bits type
    // i means signed integer bits type
    // f means the float bits type 

    //******************OVERFLOW******************//
    
    //this will overflow beacuse the bits are not valid for larger values then defiend
    // eg. i or u 255 can be used to store large int.
    // Below is the example:

    // let x: u8 = 256; //0 - 225 
    // let y: i8 = 10; //-128 - 127 
    
    //******************MIX TYPING******************//

    //here this will cause an error because the signed and unsigned integer cannot be added directly
    // Below is the example:
    
    // let x: u8 = 12; //0 - 225 this will overflow    
    // let y: i8 = 10; //-128 - 127 this will overflow

    //******************ARITHMETIC OPERATIONS******************//

    //IMP: you must convert the following bases together and the implement arth operations.
    //IMP: this can only work if the division / operation is implemented.
    //IMP: this can only work if the multiplication * operation is implemented.
    //IMP: this can only work if the modulus % operation is implemented.
    // Below is the example:

    //FOR MULTIPLICATION, DIVISION AND MODULUS
    // let x: f64 = 255.0; 
    // let y: f64 = 10.0;

    //let z = x % y; applying mod (this will work)
    //let z = x / y; applying division (this will work)
    //let z = x * y; applying division (this will work)

    //FOR ADDITION AND SUBTRACTION
    // let x: i8 = 12;
    // let y: i8 = 10; 
    //let z = x + y; applying addtion (this will work)
    //let z = x - y; applying subtraction (this will work)

    //******************WAYS TO DEFINE TYPE CAST (BITS TYPE)******************//

    // you can also define types like:
    
    // let x: f64 = 255.0; 
    // let y: f64 = 10.0;

    // let x = 255.0f32; 
    // let y = 10.0f32;

    // let x = 127_i8; 
    // let y = 10_i8; 

    // let x = 127_i8;
    // let y = 10_i8; 

    // let x = 127 as i64;
    // let y = 10 as i32; 


    // let x = 127_0 as i64;
    // let y = 10 as i32;
    // let z = x/(y as i64); 
    //output will be 127;

    // let x = 127_000 as i64;
    // let y = 10 as i32;
    // let z = x/(y as i64); 
    //output will be 12700;

    // let x = 127 as i64;
    // let y = 10 as i32;
    // let z = x/(y as i64); 
    //output will be 12;


    //***********Coversion of bits***********
    // let x = 127 as i64;
    // let y = 10 as i32;
    // let z = x/ y as i64;

    // println!("{}",z);

// }   


//***********LECTURE 07***********\\

// fn main(){
//     let food = "fruit";

//     if food == "cookie"{
//         println!("nice");
//     } else if food == "fruit"{
//         println!("healthy")
//     } 
//     else{
//         println!("ewww");
//     }
// }
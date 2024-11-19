// Owernership Parameter 
fn ownership(s:String){
    println!("wellcome:{}",s);
}


fn main() {
    let s = String::from("Rust");
    
    ownership(s);
}

// =================================

// borrow parameter

// fn borrow_value(s:&str){
//     println!("wellcome to : {}", s);
// }

// fn main() {
//     let s = "DEAD";
    
//     borrow_value(s);
//     println!("String: {}", s);
// }

// =================================

// Return value from a function

//  fn sum(a:i32, b:i32) -> i32 {
//     a+b 
//  } 


//  fn main (){
//     let result= sum(3,5);

//     println!("The sum Is {}", result);
//  }

// ====================================

// Public Fn & Private FN In Module

// mod module1{
//     pub fn public_function(){
//         println!("This is public Function");
//     }

//     fn private_fun(){
//         println!("This is Private Fn");
//     }
// }

// fn main(){
//     module1::public_function();
//     // module1::private_fun();
// }


// ==================================

// fUNCTION OVERLOADING 

// fn greet(name: &str) {

//     println!("Hello, {}!", name);
    
// }
    
// fn greet(name: &str, age: i32) {
    
//    println!("Hello, {}! You are {} years old.", name, age);
    
// }
    
// fn main() {
    
//     greet("Alice"); // Calls the first function
        
//     greet("Bob", 30); // Calls the second function
    
// }

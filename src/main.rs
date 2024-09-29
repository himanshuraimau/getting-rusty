fn main() {
    println!("Hello, world!");
}

// } */

// /*fn main() {
//     //bolean

//         let is_male = true;
//         let is_above_18 = true;

//         if is_male {
//             println!("You are a male");

//         } else {
//             println!("You are not a male");
//         }

//         if is_male && is_above_18 {
//             print!("You are a legal male");
//         }
// }
// */

// /*

// fn main(){
//     //Strings
//     let name = "John";
//     let age = 30;
//     let sentence = format!("My name is {} and I am {}", name, age);
//     println!("{}", sentence);


//     let greeting = String::from("hello world");
//     println!("{}", greeting);

//     // print!("{}", greeting.chars().nth(1000))

// } */

// /*
// fn main(){
//     let is_even = true;

//     if is_even {
//         println!("The number is even");
//     } else {
//         println!("The number is odd");
//     }

// } */

// /*
// fn main(){
//     //loop
//     for i in 0..10 {
//         println!("i = {}", i);
//     }

// } */

// /*
// fn main(){
//     //add function
//     let sum = add(5, 5);
//     println!("sum = {}", sum);
// }

// fn add(a:i32, b:i32) -> i32 {
//     let sum = a + b;
//     return sum;
// } */


// /*
// fn main() {
//     let mut x: i32 = 1;
//    // x = 2; // Error because x is immutable
//     x = 2;
//     println!("{}", x);
// }

// */

// /*
// fn main() {
//     stack_fn();   // Call the function that uses stack memory
//     heap_fn();    // Call the function that uses heap memory
//     update_string();  // Call the function that changes size of variable at runtime
// }

// fn stack_fn() {
//     // Declare a few integers on the stack
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }

// fn heap_fn() {
//     // Create a string, which is allocated on the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }

// fn update_string() {
//     // Start with a base string on the heap
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     print!("Capacity:{},Length:{},Pointer:{:p}",s.capacity(),s.len(),s.as_ptr());
//     // Append some text to the string
//     s.push_str(" and some additional text");
//     println!("After update: {}", s);
//     print!("Capacity:{},Length:{},Pointer:{:p}",s.capacity(),s.len(),s.as_ptr());
// }
// */

// /*
// fn main(){
//     //Ownership
//     let s1 = String::from("hello");
//     println!("{}",s1);
//     let s2 = s1;
//     println!("{}",s2)
// }
// */

// fn main() {
//     let my_string = String::from("hello");
//     onefun(my_string);
//     println!("{}", my_string);
// }

// fn onefun(some_string: String) {
//     println!("{}", some_string);
// }
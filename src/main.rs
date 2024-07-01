/*fn main() {
  //  println!("Hello, world!");
    varibles
    //let i:i32 = 10;
    print!("i = {}", i);

    
} */

/*fn main() {
    //bolean
   
        let is_male = true;
        let is_above_18 = true;
        
        if is_male {
            println!("You are a male");
    
        } else {
            println!("You are not a male");
        }
    
        if is_male && is_above_18 {
            print!("You are a legal male");
        }
}
*/

/* 

fn main(){
    //Strings
    let name = "John";
    let age = 30;
    let sentence = format!("My name is {} and I am {}", name, age);
    println!("{}", sentence);

   
    let greeting = String::from("hello world");
    println!("{}", greeting);
    
    // print!("{}", greeting.chars().nth(1000))
    
} */

/* 
fn main(){
    let is_even = true;

    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

} */

/*
fn main(){
    //loop
    for i in 0..10 {
        println!("i = {}", i);
    }

} */


fn main(){
    //add function
    let sum = add(5, 5);
    println!("sum = {}", sum);
}

fn add(a:i32, b:i32) -> i32 {
    let sum = a + b;
    return sum;
}
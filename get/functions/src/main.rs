fn main() {
    println!("Hello, world!");
    another_function();
    another_function1(6);
    //let x= (let y=6); this is not allowed in rust
    // because let y=6 is a statement and it does not return a value

    let _y={
        let x=3;
        x+1
    }; // this is an expression and it returns a value so it is allowed
    // but if we put a ; at the end of x+1 then it
    //will become a statement and it will not be allowed
    println!("The value of y is: {}", _y);

    let new  = five();
    println!("The value of new is: {}", new);

}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}


fn five() -> i32 {
    5   // no ; at the end of 5 because it is an expression
    // if we put ; at the end of 5 then it will become a statement
    // and it will not return a value
}
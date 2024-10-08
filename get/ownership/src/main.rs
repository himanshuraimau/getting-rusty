fn main() {
    //ownership

    let name = String::from("John");
    let age = 30;
    let sentence = format!("My name is {} and I am {}", name, age);
    println!("{}", sentence);

    drop(name);
    // println!("{}", name);

    let s:String =  String::from("hello");
    // let s1:String = s; ownership is transferred to s1 and s is no longer valid
    // rater than doing this we can clone the value of s
     let s1 = s.clone();
    println!("{}", s);


    // ownership and functions
    let s = String::from("hello");
    //takes_ownership(s);
    // println!("{}", s); // this will throw an error because ownership has been transferred to the function
    let s = gives_ownership();

    println!("{}", s);  // this will work because ownership has been transferred back to s

    let s1 = String::from("hello");
    let s2 = gives_and_takes_ownership(s1);

    println!("{}", s2); // this will work because ownership has been transferred back to s2
    // println!("{}", s1); // this will throw an error because ownership has been transferred to s2


    // ownership and return values
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn takes_ownership(s: String){ // s comes into scope
    println!("{}", s);  // s is valid here
} // s goes out of scope here and is dropped


fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn gives_and_takes_ownership(s: String) -> String {
    s
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
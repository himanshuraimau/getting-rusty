fn main() {
    println!("Hello, world!");

    let mut age: i32 = 20;
    println!("Age: {}", age);

    age = 21;
    println!("Age: {}", age);

    const PI:u8  = 10;    // we always have to specify the type of the constant
    //also we can define const at global level
    //but we can't do it with let keyword
    // const num:u8 = age+12; // this will give error because we can't use dynamic values in const
    println!("PI: {}", PI);


    // shadowing
    let age = 22;
    let age = age + 1;
    {
        let age = age + 1;
        println!("Age: {}", age);
    }
    println!("Age: {}", age);
}

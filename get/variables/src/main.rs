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



    //data types
    let _a: i32 = 10; // 32 bit signed integer
    let _b: u32 = 10; // 32 bit unsigned integer
    let _s = 1_000_000; // this is same as 1000000

    //characters can store more than just alphabets
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    //tuples  - can store multiple values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    //arrays  - can store multiple values of same type
    let a :[u32;5]= [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    let so = [3; 5]; // this will create an array of 5 elements with all elements as 3
    println!("so: {:?}", so);
}

fn main() {
    // if is a expression

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if is a expression
    // because if is an expression, we can use it on the right side of a let statement
    // also we have to give else block because if is an expression and it should return a value
    let y = if number < 5 { 10 } else { 20 };
    println!("y: {}", y);


    // loop
    let mut counter = 0;
    loop{
        println!("again! {}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }


    let result =  loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    println!("result: {}", result);



    // loop label
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop");
            break 'outer;
        }
    }


    //while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }


    // for loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }
    // but we can use for loop because it is more concise
    // if we want to iterate over the elements of a collection in a sequence
    // we can use for loop instead of while loop because we don't have to manage the index

    for x in a{
        println!("x = {}", x);
    }

    // we can also create a ranage
    for x in (1..10).rev(){
        println!("x = {}", x);
    }

    // we can use for loop to iterate over the elements of a collection
    // we can use while loop to loop based on a condition
    // we can use loop to loop forever or loop until a break condition
    // we can use loop label to break out of a nested loop
    // we can use if as an expression

    let fib = fib(10);

    println!("fib(10) = {}", fib);
    chrismas_song();

}


fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);

    }


}

// how this works is that we have two arrays, one for the days and one for the gifts
// we iterate over the days array and for each day we iterate over the gifts array
// we print the day and the gifts
// if the day is not the first day, we print "and" before the last gift
// we print a newline after each day
fn chrismas_song(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", days[i]);
        for j in (0..i+1).rev(){
            if j == 0 && i != 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
}
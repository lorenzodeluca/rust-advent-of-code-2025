//https://www.w3schools.com/rust

fn main() {
    print!("A");
    print!("B");
    print!("C\n");
    println!("D"); // This is a comment
    println!("E");
     // This is a comment 
     /* This is a comment 
     
     */
    let name = "John";
    println!("My first name is: {}", name); 
    let age = 30;
    println!("{} is {} years old.", name, age); 
    
    //by Default variables in Rust cannot be changed after they are created
    let x = 5;
    //x = 10; // Error by Default variables in Rust cannot be changed after they are created
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x); 

    //types implicit
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string

    //explicit
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string

    
    //&str - is called "string slices", and is used for fixed text like "Hello"
    //String - used when you need a string that can change
    let text1 = "Hello World".to_string();  //You can create a String from a string literal using the to_string() method or the String::from() function:
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");

    //to combine strings:
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    let result = s1 + " " + &s2 + " " + &s3;
    //Note: You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here.
    println!("{}", result); 

    //string length
    let name = String::from("John");
    println!("Length: {}", name.len()); // 4 

    // constants type must always be explicit. var name should be uppercase
    const BIRTHYEAR: i32 = 1980; 

    //operators same as c++/Java
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);


    // conditions
    if my_bool {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    if 7 > 5 {
        println!("7 is greater than 5.");
    } 

    let time = 20;
    let greeting = if time < 18 { "Good day." } else { "Good evening." };
    println!("{}", greeting);

    //let result = if number < 10 { "Too small" } else { 100 }; //error

    // switch -> match
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    //multiple match
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    //match with return value
    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    //loops

    //loop {
    //    println!("This will repeat forever!");
    //}
    
    let mut count = 1;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }
    
    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };


    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
        // inside while 'break' can be used
        // You can skip a value by using the 'continue' statement:
    } 

    for i in 1..6 { //6 escluded, i>=1 && i<6
        println!("i is: {}", i);
    } 

    for i in 1..=6 { //i>=1 && i<=6
        println!("i is: {}", i);
    } 

    // Create a function
    fn say_hello() {
        println!("Hello from a function!");
    }

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    say_hello(); // Call the function 

    greet("John");

    //function with return value
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    let sum = add(3, 4);


    let x = 5;
    let x = 10; //shadowing

    println!("x is: {}", x); // prints 10 


    //---Ownership Rules    
    //- Each value has one owner
    //- When the owner goes out of scope, the value is deleted
    //- You can only have one owner at a time, unless you borrow it
    let a = String::from("Hello");
    let b = a;

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value 

    //simple types like numbers, characters and booleans are copied, not moved.
    let a = 5;
    let b = a;
    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works 

    //cloning is possible
    //For other types, like String, if you really want to keep the original value and also assign it to another variable, you can use the .clone() method, which makes a copy of the data:
    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value

    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works 

    //-- Borrowing and References
    //to use a value without taking ownership of it
    //A reference lets you look at a value without owning it. You create a reference using the & symbol
    //If you want to change a value through a reference, you need to make the reference mut:
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref); // John Doe 
}





fn main() {
    // Rust convention for writing consts
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    // Assigning different types to values in a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Tuple destructuring
    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    
    // a tuple without any values is called a unit - written ()

    // accessing tuple element directly with indexing
    let z: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = z.0;
    let six_point_four = z.1;
    let one = z.2;
    
    // all values in an array has to be of the same data type and fixed length
    // arrays are useful when you know the number of elements will not change
    // here we declare the type of the array to be i32 and length to be 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // this array will contain 5 elements of the value 3. - [3, 3, 3, 3, 3]
    let a = [3; 5];

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
    }

    // expressions do not include ending semicolons
    // numbers by themselves are also expressions
    // by adding a semicolon at the end of an expression, you turn it into a statement and it will not return a value
    // expressions - the expression here is { let x = 3; x + 1 }
    let u = {
        let x = 3;
        x + 1
    };

    another_function(55, 'h');
    let f = five();
    let p1 = plus_one(f);
    println!("p1 is: {p1}");

    // if statements:
    // rust does not automatically convert non-Boolean types to Boolean. You must be explicit and always provide if with a Boolean as its condition
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number_test = if condition { 5 } else { 6 };

    // loops:
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // nested loops:
    let mut count = 0;
    // giving the loop a label
    'counting_up: loop {
        // println!("count = {count}");
        let mut remaining = 10;
        loop {
            // println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // exiting the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    // println!("End count = {count}");

    // looping through an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while loop
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for loop
    for element in a {
        println!("the value is: {element}");
    }
    // .. in range -- .rev() is used to reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // converting degrees in fahrenheit to celsius
    let parameter_degrees_f: f64 = 40.0;
    let c = convert_to_celsius(parameter_degrees_f);
    // println!("{parameter_degrees_f} Fahrenheit is {c} degrees celsius");


    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // memory and allocation
    let s1 = String::from("hello");
    let s2 = s1;
    // To ensure memory safety - s1 is no longer accessible. s1 has been moved into s2
    // because s1 was moved, the problem with double free error (one of the memory safety bugs) is solved
    // println!("{}, world!", s1);
    // if we did want to copy the heap data of the String, not just the stack data, we could use a common method called clone. -- let s2 = s1.clone();

}
// ownership and functions:
fn test() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn test2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
                                        
    let s2 = String::from("hello");     // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

// in function signatures you must declare the type of each parameter
fn another_function(value: i32, unit_label: char) {
    println!("The meassurement is: {value}{unit_label}");
}

// we dont name return values, but we must declare their type after an arrow
// you can return early by using the return keyword
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn convert_to_celsius(f: f64) -> f64 {
    (f-32.0) * (5.0/9.0)
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> (usize, usize) {
    // string slices
    let s = String::from("hello world");

    // hello is a reference to a portion of the String specified in the extra [..5] bit
    let hello = &s[..5];
    let world = &s[6..]

    let len = s.len(); 

    // these are equal:
    let slice = &s[0..len];
    let slice = &s[..];

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // slicing part of an array:
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


// structs:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// to use a struct after we've defined it, we create an instance of that struct by specifying concrete values for each of the fields
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
// to access email for example - in this case use: user1.email. If the instance is mutable, we can change a value by using the dot notation and assigning it to a particular field
user1.email = String::from("anotheremail@example.com");
// either an entire struct is mutable or none of it is. Certain fields can not be mutable if others aren't. 
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
// using the field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email, // this is the same as email: email,
        username, // this is the same as username: username,
        active: true,
        sign_in_count: 1,
    }
}
// creating instances from other instances with struct update syntax
// .. this is the same as ..
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
// .. same as this:
let user2 = User {
    email: String::from("another@example.com"),
    // the ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1
    ..user1
};

// tuple structs:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
// structs without fields - unit-like structs:
struct AlwaysEqual;

let subject = AlwaysEqual;

// debugging:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn debug_test() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // :? syntax is for allowing us to pretty-print
    println!("rect1 is {:#?}", rect1);
    // can also use dbg! macro instead of println! macro when debugging. It takes ownership of an expression (as opposed to println! that takes a referance)
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn debug_test_2() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
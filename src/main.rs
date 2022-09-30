fn main() {
    // Rust convention for writing consts
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    
    // Assigning different types to values in a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Tuple destructuring
    let tuple = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("The value of y is: {y}");
    
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
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

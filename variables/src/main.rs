

fn main() {
    // ----- Variables and Mutability ----- //
    let x: i32 = 5;

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces: &str = "    ";
    let spaces: usize = spaces.len();
    println!("The number of spaces is: {spaces}");

    // ----- Data Types ----- //

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is: {guess}");

    let f: bool = false;
    println!("The value store in f is {f}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The numbers in the tuple are: {}, {}, {}", x, y, z);
    println!("The numbers in the tuple are: {}, {}, {}", tup.0, tup.1, tup.2);

    let arr: [i32; 5] = [1,2,3,4,5];
    let first: i32 = arr[0];
    println!("The first value in the array is: {first}");

    // ----- Functions ----- //

    another_function(first);


}

fn another_function(x: i32){
    println!("Another Function!");
    println!("The value of x is {x}");
}



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
    print_labeled_measurements(5, 'h');

    // ------ Statements and Expressions ----- //
    let a = {
        let b = 3;
        b + 1 // No semicolon
    };
    println!("The value of a is: {a}");

    // ----- Functions with Return Values ----- //
    
    println!("The return of funcion five is {}", five());
    println!("The combination of these two functions is {}", plus_one(five()));

    // ----- Control Flow ----- //
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

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is {}", number);

    // ----- Repitition with Loops ----- //

    let mut count = 0;
    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let a: [i32; 5] = [10,20,30,40,50];
    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..5).rev() {
        print!("{number}!");
    }

    println!("\ngit test");

}


// ----- Functions ----- //
fn another_function(x: i32){
    println!("Another Function!");
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// ----- Functions with Return Values ----- //

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
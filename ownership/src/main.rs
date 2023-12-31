
fn main() {
    println!("cargo:rustc-link-lib=dylib=OpenCL");
    println!("cargo:rustc-link-search=native=C:\\OpenCL-SDK\\install\\lib\\OpenCL.lib");
    // ----- What is ownership ----- //
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    
    let s2 = s1;
    let s3 = s2.clone();

    let x = 5;
    let y = x;

    println!("{} {}", s2, s3);
    println!("{} {}", x, y);

    takes_ownership(s2);
    makes_copy(x);
    let s2 = gives_ownership();
    let s3 = takes_and_gives_back(s2);

    println!("{s3}");

    let (s4, len) = calculate_length(s3);
    println!("{s4}, {len}");

    // ----- References andBorrowing ----- //

    let s1 = String::from("hello");
    let len = calculate_length_2(&s1);
    println!("{} {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    
    let s = no_dangle();
    println!("{s}");


    // ----- The Slice Tyoe ----- //
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("the first word is: {}", word);

    let my_string = String::from("Hello World");
    let word = first_word2(&my_string);
    println!("the first word is {}", word);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}


// ----- What is ownership functions ----- //
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer + 1);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// ----- Reference and Borrowing Functions ----- //
fn calculate_length_2 (s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// ----- The Slice Type Functions ----- //

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
        
    &s[..]
}
fn first_word2 (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
        
    &s[..]
}

// `derive` will automatically creates the implementation of `std::fmt::Debug` which required to make this struct printable

// For the example in formatted_print.rs. This structure did not create any implementation to make this struct printable
// => it would fail while compiling to binary file
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
struct Deep(Structure);

// {:?} auto implement std library for printable
fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    // But if use `{}` instead of `{:?}` here. It will be an error while combine because there is no implementation of `std::fmt::Display`
    // But `Display` trait can only be implemented manually (cannot use derive). See https://practice.rs/formatted-output/debug-display.html
    println!("Now {:?} will print!", Structure(3));
    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // Pretty print `{:#?}`
    println!("Now {:#?} will print!", Structure(3));
}
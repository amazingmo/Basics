// Ordinary comments in Rust are the double slash variety.
// Documentation comments are three slashes.

use std::io::Write;
/// Write is a trait. Any type that implements this trait has
/// a write_fmt method. This is needed by the writeln! macro.
/// In this case, we're going to apply it to std::io::stderr

use std::str::FromStr;
/// FromStr is a trait. Any type that implements this trait has
/// a from_str method. Since u64 implements FromStr, we can
/// use u64::from_str()

// If you name an external crate, then you also have to
// list it, (optionally with a required version) in the
// cargo.toml manifest.
// '#[macro_use]' means import macros as well as real elements.

extern crate iron;
#[macro_use] extern crate mime;

// Bring names from external crates into local namespace
// Modules named "prelude" are intended to be included using '*'
// by convention.
use iron::prelude::*;
use iron::status;


/// The main function doesn't return a value.
/// Other functions have a trailing type indicator to tell
/// what the return type is. (With the exception of special
/// functions that aren't supposed to return.)
fn main()
{
    // Page 45 of the crab book has the basic types and literals.
    // Here are a few interesting examples.
    
    // Prefacing variable names with _ makes the compiler not 
    // care that the value is not used.
    // Almost all of these examples could have been written without
    // the type specifier and Rust would have inferred the type.
    let _byte : u8 = b'*';
    let _ptr : usize = 0xffff_ffff_usize; // NB trailing type specifier in literal.
    let mut _is_open = false; // implied type bool.
    let _pi : f32 = 3.14_f32; // Arbitrary underscores are allowed.
    let _c : char = '\u{CA0}'; // chars are 4byte unicode points.
    // Strings are UTF-8 sequences...
    let _s : String = "the literal is not a string, but is converted".to_string();
    // Literals and slices are non-owning references to str, a different type.
    let _not_s : &str = "this is not a string.";
    let mut _not_s_either : [u8; 64] = [0x0; 64];
    let _fn_ptr : fn (i32, i32) -> i32 = i32::saturating_add;
    
    // Structs, enums, boxes, tuples need their own section.
    // There are also vectors, "references to slices", trait objects
    // (these look like any other reference to me), and closures (whose type
    // is a secret of the compiler.)

    let mut numbers = Vec::new(); // new is a static factory function of the type.

    // Raw strings in Rust don't have any escape sequences. The number of #
    // symbols before the opening " needs to match the number after the closing
    // quote for the closing quote to be recognised.  It can be any number,
    // including zero.
    let msg = r#"Error parsing the argument"#;
    
    // std::env::args() produces an iterator.
    for arg in std::env::args().skip(1)
    {
        // .expect is called on the Result type that is returned by
        // u64::from_str.
        // The '&' in &arg means borrow a non-mutable reference
        numbers.push(u64::from_str(&arg).expect(msg));
    }

    if numbers.len() == 0
    {
        // .unwrap() checks that the write didn't fail. Could have used .expect(msg)
        writeln!(std::io::stderr(), "Usage: gcd <UINT>+").unwrap();
        std::process::exit(1);
    }
    
    
    let mut d = numbers[0];
    
    // &numbers[1..] is a bit weird.
    // The '&' means borrow a non-mutable reference to...
    // ... in this case each of the values of numbers from
    // the second (zero indexing) to the end.
    //
    // The type is "non-owning reference to slice."
    for m in &numbers[1..]
    {
        // Using *m to dereference the borrowed reference.
        d = gcd(d, *m);
    }
    
    println!("The greatest common divisor of {:?} is {}", numbers, d);

    // Use an underscore for a variable name when you don't care about it.
    let _ = gcd(2, 3);
}

/// fn introduces a function
/// mut is the keyword that means unconst. If it is missing, the variable is a constant.
/// u64 is an unsigned 64bit integer.
/// Use usize for an unsigned integer the same size as a pointer.
fn gcd (mut n: u64, mut m: u64) -> u64
{
    // assert! is a macro. The exclamation mark indicates the "macroness".
    assert!(n != 0 && m != 0); // semicolons mean something
    while m != 0
    {
        if m < n
        {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // Missing the semicolon here means that the return value of this function is the value of n.
}

// The "#[test]" is an example of an attribute.
#[test]
fn test_gcd()
{
    assert_eq!(gcd(14, 15), 1);
    
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}
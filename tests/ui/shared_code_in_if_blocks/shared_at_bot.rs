#![allow(dead_code)]
#![deny(clippy::if_same_then_else, clippy::shared_code_in_if_blocks)]

// This tests the shared_code_in_if_blocks lint at the end of blocks

fn simple_examples() {
    // TODO xFrednet 2021-01-06: Test with const literals at the end
    let x = 1;

    let _ = if x == 7 {
        println!("Branch I");
        let start_value = 0;
        println!("=^.^=");

        // Same but not moveable due to `start_value`
        let _ = start_value;

        // The rest is self contained and moveable => Only lint the rest
        let result = false;
        println!("Block end!");
        result
    } else {
        println!("Branch II");
        let start_value = 8;
        println!("xD");

        // Same but not moveable due to `start_value`
        let _ = start_value;

        // The rest is self contained and moveable => Only lint the rest
        let result = false;
        println!("Block end!");
        result
    };

    // Else if block
    if x == 9 {
        println!("The index is: 6");

        println!("Same end of block");
    } else if x == 8 {
        println!("The index is: 4");

        // We should only get a lint trigger for the last statement
        println!("This is also eq with the else block");
        println!("Same end of block");
    } else {
        println!("Same end of block");
        println!("This is also eq with the else block");
    }

    // Use of outer scope value
    let outer_scope_value = "I'm outside the if block";
    if x < 99 {
        let z = "How are you";
        println!("I'm a local because I use the value `z`: `{}`", z);

        println!(
            "I'm moveable because I know: `outer_scope_value`: '{}'",
            outer_scope_value
        );
    } else {
        let z = 45678000;
        println!("I'm a local because I use the value `z`: `{}`", z);

        println!(
            "I'm moveable because I know: `outer_scope_value`: '{}'",
            outer_scope_value
        );
    }

    // TODO xFrednet 2021-01.13: Fix lint for `if let`
    let index = Some(8);
    if let Some(index) = index {
        println!("The index is: {}", index);

        println!("Same end of block");
    } else {
        println!("Same end of block");
    }

    if x == 9 {
        if x == 8 {
            // No parent!!
            println!("Hello World");
            println!("---");
        } else {
            println!("Hello World");
        }
    }
}

/// Simple examples where the move can cause some problems due to moved values
fn simple_but_suggestion_is_invalid() {
    let x = 16;

    // Local value
    let later_used_value = 17;
    if x == 9 {
        let _ = 9;
        let later_used_value = "A string value";
        println!("{}", later_used_value);
    } else {
        let later_used_value = "A string value";
        println!("{}", later_used_value);
        // I'm expecting a note about this
    }
    println!("{}", later_used_value);

    // outer function
    if x == 78 {
        let simple_examples = "I now identify as a &str :)";
        println!("This is the new simple_example: {}", simple_examples);
    } else {
        println!("Separator print statement");

        let simple_examples = "I now identify as a &str :)";
        println!("This is the new simple_example: {}", simple_examples);
    }
    simple_examples();
}

/// Tests where the blocks are not linted due to the used value scope
fn not_moveable_due_to_value_scope() {
    let x = 18;

    // Using a local value in the moved code
    if x == 9 {
        let y = 18;
        println!("y is: `{}`", y);
    } else {
        let y = "A string";
        println!("y is: `{}`", y);
    }

    // Using a local value in the expression
    let _ = if x == 0 {
        let mut result = x + 1;

        println!("1. Doing some calculations");
        println!("2. Some more calculations");
        println!("3. Setting result");

        result
    } else {
        let mut result = x - 1;

        println!("1. Doing some calculations");
        println!("2. Some more calculations");
        println!("3. Setting result");

        result
    };

    let _ = if x == 7 {
        let z1 = 100;
        println!("z1: {}", z1);

        let z2 = z1;
        println!("z2: {}", z2);

        z2
    } else {
        let z1 = 300;
        println!("z1: {}", z1);

        let z2 = z1;
        println!("z2: {}", z2);

        z2
    };
}

#[rustfmt::skip]
fn test_suggestion_with_weird_formatting() {
    let x = 9;
    let mut a = 0;
    let mut b = 0;

    // The error message still looks weird tbh but this is the best I can do
    // for weird formatting
    if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }
}

fn main() {}

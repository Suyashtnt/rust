// Checks that there is a suggestion for simple tuple index access expression (used where an
// identifier is expected in a format arg) to use positional arg instead.
// Issue: <https://github.com/rust-lang/rust/issues/122535>.
//@ run-rustfix

fn main() {
    let x = (1,);
    println!("{x.0}");
    //~^ ERROR invalid format string
}

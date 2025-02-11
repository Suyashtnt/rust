//@ run-rustfix

use std::fmt;

struct Hello;

impl fmt::Display for Hello {
    fn fmt(&self, f: &mut fmt:Formatter) -> fmt::Result { //~ ERROR path separator must be a double colon
        write!(f, "hello")
    }
}

fn main() {
    let _ = Hello;
}

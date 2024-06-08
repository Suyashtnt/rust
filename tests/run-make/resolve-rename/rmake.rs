// If a library is compiled with -C extra-filename, the rust compiler
// will take this into account when searching for libraries. However,
// if that library is then renamed, the rust compiler should fall back
// to its regular library location logic and not immediately fail to find
// the renamed library.
// See https://github.com/rust-lang/rust/pull/49253

use run_make_support::rustc;
use std::fs;
fn main() {
    rustc().extra_filename("-hash").input("foo.rs").run();
    rustc().input("bar.rs").run();
    fs::rename("libfoo-hash.rlib", "libfoo-another-hash.rlib").unwrap();
    rustc().input("baz.rs").run();
}

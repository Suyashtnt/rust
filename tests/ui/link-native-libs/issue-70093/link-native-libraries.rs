//@ run-pass
//@ compile-flags: -Zlink-native-libraries=no -Cdefault-linker-libraries=yes
//@ ignore-fuchsia - missing __libc_start_main for some reason (#84733)
//@ ignore-cross-compile - default-linker-libraries=yes doesn't play well with cross compiling

//@ revisions: other
//@[other] ignore-msvc

//@ revisions: msvc
// On Windows MSVC, default-linker-libraries=yes doesn't work because
// rustc drives the linker directly instead of going through another compiler.
// Therefore rustc would need to implement default-linker-libraries itself but doesn't.
// So instead we use -Clink-arg to directly set the required msvcrt.lib as a link arg.
//@[msvc] compile-flags: -Clink-arg=msvcrt.lib
//@[msvc] only-msvc

#[link(name = "some-random-non-existent-library", kind = "static")]
extern "C" {}

fn main() {}

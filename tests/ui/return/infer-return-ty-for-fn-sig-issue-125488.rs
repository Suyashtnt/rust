//@ run-rustfix

#[allow(dead_code)]

fn main() {
    struct S<'a>(&'a ());

    fn f1(s: S<'_>) -> _ {
        //~^ ERROR the placeholder `_` is not allowed
        s
    }

    fn f2(s: S<'_>) -> _ {
        //~^ ERROR the placeholder `_` is not allowed
        let x = true;
        if x {
            s
        } else {
            s
        }
    }

    fn f3(s: S<'_>) -> _ {
        //~^ ERROR the placeholder `_` is not allowed
        return s;
    }

    fn f4(s: S<'_>) -> _ {
        //~^ ERROR the placeholder `_` is not allowed
        let _x = 1;
        return s;
    }
}

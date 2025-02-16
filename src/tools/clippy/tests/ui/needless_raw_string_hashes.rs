#![allow(clippy::no_effect, unused)]
#![warn(clippy::needless_raw_string_hashes)]

fn main() {
    r#"\aaa"#;
    r##"Hello "world"!"##;
    r######" "### "## "# "######;
    r######" "aa" "# "## "######;
    br#"\aaa"#;
    br##"Hello "world"!"##;
    br######" "### "## "# "######;
    br######" "aa" "# "## "######;
    cr#"\aaa"#;
    cr##"Hello "world"!"##;
    cr######" "### "## "# "######;
    cr######" "aa" "# "## "######;

    r#"
        \a
        multiline
        string
    "#;

    r###"rust"###;
    r#"hello world"#;
}

fn issue_13503() {
    println!(r"SELECT * FROM posts");
    println!(r#"SELECT * FROM posts"#);
    println!(r##"SELECT * FROM "posts""##);
    println!(r##"SELECT * FROM "posts""##);

    // Test arguments as well
    println!("{}", r"foobar".len());
}

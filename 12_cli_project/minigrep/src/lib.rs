// 'a: "Returned &str in Vec are only valid as long as contents (param) are"
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

// TODO: left off https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html


// 'a: "Returned &str in Vec are only valid as long as contents (param) are"
// data returned by search will live as long as data in "contents" param
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

// TODO: left off https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html#iterating-through-lines-with-the-lines-method

#[cfg(test)]
mod tests {
   use super::*; 

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

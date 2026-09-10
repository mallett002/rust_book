// 'a: "Returned &str in Vec are only valid as long as contents (param) are"
// data returned by search will live as long as data in "contents" param
pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    // let mut result = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    //
    // result

    let query = query.to_string();

    contents.lines().filter(move |l| l.contains(&query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();

    // let mut result = Vec::new();
    //
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }
    //
    // result

    contents
        .lines()
        .filter(move |l| l.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // shouldn't find "Duct" - case sensitive

        let results = search(query, contents);
        let results_vec: Vec<&str> = results.collect();

        assert_eq!(vec!["safe, fast, productive."], results_vec);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results = search_case_insensitive(query, contents);
        let results_vec: Vec<&str> = results.collect();

        assert_eq!(
            vec!["Rust:", "Trust me."],
            results_vec
        );
    }
}

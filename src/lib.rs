pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(&query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))
}

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
        let mut v = vec![];
        for val in search(query, contents) {
            v.push(val);
        }
        assert_eq!(vec!["safe, fast, productive."], v);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let mut v = vec![];
        for val in search(query, contents) {
            v.push(val);
        }
        assert_eq!(vec!["safe, fast, productive."], v);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let mut v = vec![];
        for val in search_case_insensitive(query, contents) {
            v.push(val);
        }
        assert_eq!(vec!["Rust:", "Trust me."], v);
    }
}

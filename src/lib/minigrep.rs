use std::fs;

pub fn open_file(path: &str) -> Result<String, String> {
    let content = fs::read_to_string(path);
    match content {
        Ok(s) => Ok(s),
        Err(err) => Err(format!("Unable to open {}\nError: {}", path, err)),
    }
}

// will write tests for this when I learn how to mock functions
pub fn search<'a>(needle: &str, haystack: &'a str, ignore_case: bool) -> Option<Vec<&'a str>> {
    let result: Vec<&str>;
    if ignore_case {
        result = search_case_insensitive(&needle, &haystack);
    } else {
        result = search_case_sensitive(&needle, &haystack);
    }

    if result.len() > 0 {
        Some(result)
    } else {
        None
    }
}

fn search_case_sensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let lower_needle = needle.to_lowercase();
    for line in haystack.lines() {
        if line.to_lowercase().contains(&lower_needle) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_needle() {
        let needle1 = "malesuada";
        let needle2 = "Pellentesque";

        let haystack = "\
Pellentesque non malesuada lorem.
Ut in porttitor nisl. Suspendisse vitae tempus arcu.
Pellentesque scelerisque lorem id ex laoreet laoreet.";

        assert_eq!(
            vec!["Pellentesque non malesuada lorem."],
            search_case_sensitive(needle1, haystack)
        );
        assert_eq!(
            vec![
                "Pellentesque non malesuada lorem.",
                "Pellentesque scelerisque lorem id ex laoreet laoreet."
            ],
            search_case_sensitive(needle2, haystack)
        );
    }

    #[test]
    fn should_find_needle_case_insensitive() {
        let needle = "rUsT";

        let haystack = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(needle, haystack)
        );
    }
}

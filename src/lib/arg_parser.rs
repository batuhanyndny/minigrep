use std::env;

pub struct MinigrepArgs {
    pub needle: String,
    pub haystack: String,
    pub ignore_case: bool,
}

impl MinigrepArgs {
    pub fn parse(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("Invalid usage, usage: IGNORE_CASE=1 minigrep needle haystack.txt");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(MinigrepArgs {
            needle: (args[1].clone()),
            haystack: (args[2].clone()),
            ignore_case: (ignore_case),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_parse() {
        let success = vec![
            String::from("minigrep"),
            String::from("needle"),
            String::from("haystack.txt"),
        ];
        let fail = vec![String::from("minigrep"), String::from("needle")];
        let fail2 = vec![String::from("minigrep")];
        let fail3 = vec![
            String::from("minigrep"),
            String::from("needle"),
            String::from("haystack.txt"),
            String::from("minigrep"),
            String::from("hay"),
            String::from("cow"),
        ];

        let s = MinigrepArgs::parse(success).unwrap();
        assert_eq!(s.needle, "needle");
        assert_eq!(s.haystack, "haystack.txt");

        let f1 = MinigrepArgs::parse(fail);
        match f1 {
            Ok(_) => panic!("should have failed"),
            Err(err) => assert_eq!(err, "Invalid usage, usage: minigrep needle haystack.txt"),
        }
        let f2 = MinigrepArgs::parse(fail2);
        match f2 {
            Ok(_) => panic!("should have failed"),
            Err(err) => assert_eq!(err, "Invalid usage, usage: minigrep needle haystack.txt"),
        }

        let f3 = MinigrepArgs::parse(fail3);
        match f3 {
            Ok(_) => panic!("should have failed"),
            Err(err) => assert_eq!(err, "Invalid usage, usage: minigrep needle haystack.txt"),
        }
    }
}

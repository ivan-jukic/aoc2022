///
/// Files related extras
pub mod files {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub type FileLines = io::Lines<io::BufReader<File>>;

    /// Reads lines from a file!s
    pub fn read_lines<P>(filename: P) -> FileLines
    where
        P: AsRef<Path>,
    {
        let file_res = File::open(filename);
        match file_res {
            Ok(file) => io::BufReader::new(file).lines(),
            Err(err) => super::logging::log_and_panic(err.to_string()),
        }
    }

    pub fn read_into_vec_of<P, F, T>(filename: P, decoder: F) -> Vec<T>
    where
        P: AsRef<Path>,
        F: Fn(String) -> Option<T>,
    {
        read_lines(filename)
            .into_iter()
            .fold(vec![], |mut vals, curr| match curr {
                Ok(line) => match decoder(line) {
                    Some(v) => {
                        vals.push(v);
                        vals
                    }
                    None => vals,
                },
                Err(_err) => vals,
            })
    }
}

///
/// Logging related extras
pub mod logging {
    pub fn log_and_panic(err: impl Into<String>) -> ! {
        let err_str: String = err.into();
        log::error!("{}", err_str);
        panic!()
    }

    pub fn err_to_str(err: impl ToString) -> String {
        err.to_string()
    }
}

///
/// String related extras
pub mod strings {
    ///
    /// Break up a string into chunks of usize `n`.
    pub fn take_every_n(n: usize, haystack: impl Into<String>, res: Vec<String>) -> Vec<String> {
        let source: String = haystack.into();
        let mut new_res = res.clone();
        if source.is_empty() {
            new_res
        } else if source.len() <= n {
            new_res.push(source);
            new_res
        } else {
            let (left, other) = source.split_at(n);
            new_res.push(left.to_string());
            take_every_n(n, other, new_res)
        }
    }
}

///
/// Vec related extras
pub mod vec {
    pub fn uncons(vals: Vec<String>) -> Option<(String, Vec<String>)> {
        let mfst = vals.get(0);
        match (mfst, vals.len()) {
            (Some(fst), 1) => Some((fst.to_string(), vec![])),
            (Some(fst), _) => Some((fst.to_string(), vals[1..].to_vec())),
            _ => None,
        }
    }
}

///
/// TESTS
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_take_every_n() {
        let res = super::strings::take_every_n(4, "Hello World!!", vec![]);
        let res2 = super::strings::take_every_n(20, "Hello World!!", vec![]);
        let res3 = super::strings::take_every_n(100, "", vec![]);
        assert_eq!(res[0], "Hell");
        assert_eq!(res[1], "o Wo");
        assert_eq!(res[2], "rld!");
        assert_eq!(res[3], "!");
        assert_eq!(res2[0], "Hello World!!");
        assert_eq!(res3.len(), 0);
    }
}

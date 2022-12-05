///
/// Files related functionality
///
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
/// Logging related functionality
///
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

use std::{fs, env, path::Path, ffi::OsStr};

pub fn read_file(file_path: &str) -> String
{
    let file_ex = prog().unwrap() + "\\" + file_path;
    // println!("{}", file_ex);
    let file = if let Ok(input) = fs::read_to_string(file_path) { input } else { fs::read_to_string(file_ex).unwrap() };
    return file;
}

fn prog() -> Option<String> {
    if cfg!(windows) {  
        env::args().next()
            .as_ref()
            .map(Path::new)
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .map(String::from)
            .unwrap()
            .strip_suffix(".exe")
            .map(String::from)
    } else {
        env::args().next()
            .as_ref()
            .map(Path::new)
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .map(String::from)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

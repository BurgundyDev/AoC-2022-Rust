use std::fs;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_file(file_path: &str) -> String
{
    let file = if let Ok(input) = fs::read_to_string(file_path) { input } else { String::from("Loading failed!") };
    return file;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

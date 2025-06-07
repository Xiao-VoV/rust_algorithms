mod bin_files;
use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

pub fn write_file(path: &str, content: &str) {
    fs::write(path, content).expect("Unable to write file");
}

mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let content = read_file("/Users/xiao/Code/rust/rust_algorithms/src/files/test.txt");
        print!("{}", content);
        assert_eq!(content, "Hello, world!\n");
    }

    #[test]
    fn test_write_file() {
        let path = "output.txt";
        write_file(path, "Test content");
        let content = read_file(path);
        assert_eq!(content, "Test content");
        fs::remove_file(path).expect("Failed to clean up test file");
    }
}

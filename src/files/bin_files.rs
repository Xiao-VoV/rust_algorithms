use std::fs;
#[warn(unused)]
// 获取文件元数据
pub fn read_metadata(path: &str) -> Result<(), std::io::Error> {
    let content = fs::metadata(path).map_err(|e| {
        print!("{:?}", e);
        e
    })?;
    println!("{:?}", content);
    Ok(())
}

pub mod test {
    use super::*;
    #[test]
    pub fn test_read_bin_files() -> () {
        read_metadata("/Users/xiao/Code/rust/rust_algorithms/src/files");
    }
}

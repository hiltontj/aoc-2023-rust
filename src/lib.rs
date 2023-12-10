pub mod util {
    use std::{fs::File, io::Read, path::Path};

    pub fn file_to_string<P: AsRef<Path>>(path: P) -> String {
        let mut file = File::open(path).expect("valid file path");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("read file to string");
        contents
    }
}

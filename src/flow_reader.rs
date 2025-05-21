use std::fs;
use std::path::Path;

pub fn load_flow_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(&path);
    match content {
        Ok(content) => {
            let steps = content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect();
            Ok(steps)
        }
        Err(e) => {
            Err(e)
        }
    }
}

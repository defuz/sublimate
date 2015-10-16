#[derive(Debug)]
pub struct BuildSystem {
    cmd: Vec<String>,
    file_regex: Option<String>,
    selector: Option<String>,
}

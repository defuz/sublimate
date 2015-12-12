fn main() {
    let mut data = match Json::from_str(&line.unwrap()) {
    Ok(x) => x,
    Err(e) => panic!("Err {} !!!", e)
    };
}

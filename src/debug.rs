
pub struct Web {}
impl Web {
    pub fn error(e: reqwest::Error, message: &str) {
        println!("HTTP ERROR: {e}\n{message}")
    }
}

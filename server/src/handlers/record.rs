#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Record {
    pub level: String,
    pub timestamp: u64,
    pub text: String,
}

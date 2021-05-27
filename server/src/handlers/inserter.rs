use super::record::Record;
use jsonrpc::Handler;

pub struct Inserter(pub sled::Db);

#[async_trait::async_trait]
impl Handler for Inserter {
    type Input = Input;

    // Timestamp
    type Output = u64;

    type Error = String;

    const METHOD: &'static str = "insert";

    async fn handle(&self, params: Self::Input) -> Result<Self::Output, Self::Error> {
        let key = uuid::Uuid::new_v4();
        use std::time;
        let timestamp = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
        let record = Record {
            level: params.level,
            timestamp,
            text: params.text,
        };
        let json_record = serde_json::to_string(&record).map_err(|err| err.to_string())?;
        self.0
            .insert(key.as_bytes(), json_record.as_bytes())
            .map_err(|err| err.to_string())?;
        Ok(timestamp)
    }
}

#[derive(serde::Deserialize)]
pub struct Input {
    level: String,
    text: String,
}

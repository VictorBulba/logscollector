use super::record::Record;
use jsonrpc::Handler;
use std::ops::Range;

pub struct Seeker(pub sled::Db);

#[async_trait::async_trait]
impl Handler for Seeker {
    type Input = Input;

    type Output = Vec<Record>;

    type Error = ();

    const METHOD: &'static str = "seek";

    async fn handle(&self, params: Self::Input) -> Result<Self::Output, Self::Error> {
        let records = self
            .0
            .iter()
            .flatten()
            .flat_map(|(_, record)| serde_json::from_slice(&record))
            .filter(|record| is_matched_requirements(record, &params))
            .collect();
        Ok(records)
    }
}

fn is_matched_requirements(record: &Record, requirements: &Input) -> bool {
    requirements.timestamp.contains(&record.timestamp) && requirements.level == record.level
}

#[derive(serde::Deserialize)]
pub struct Input {
    level: String,
    timestamp: Range<u64>,
}

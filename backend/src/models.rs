use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRecord {
    pub id: u64,
    pub entity_name: String,
    pub amount: f64,
    pub status: RecordStatus,
    pub owner: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecordStatus {
    Active,
    Archived,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub page_size: usize,
    pub strategy: PaginationStrategy,
    pub output_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PaginationStrategy {
    Offset,
    Cursor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPage {
    pub records: Vec<ExportRecord>,
    pub page_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorToken {
    pub last_seen_id: u64,
    pub last_timestamp: u64,
    pub page_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatus {
    pub pages_written: usize,
    pub total_records: usize,
    pub cursor: CursorToken,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub total_records: usize,
    pub pages_written: usize,
}

pub fn generate_sample_dataset(count: usize) -> Vec<ExportRecord> {
    (0..count)
        .map(|i| ExportRecord {
            id: i as u64,
            entity_name: format!("entity_{:04}", i),
            amount: (i as f64) * 42.5,
            status: match i % 3 {
                0 => RecordStatus::Active,
                1 => RecordStatus::Pending,
                _ => RecordStatus::Archived,
            },
            owner: format!("user_{}", i % 10),
            created_at: 1700000000 + (i as u64 / 5),
        })
        .collect()
}

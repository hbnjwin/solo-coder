use crate::models::{CursorToken, ExportRecord};

pub struct PageResult {
    pub records: Vec<ExportRecord>,
    pub next_token: Option<CursorToken>,
}

pub fn paginate_by_page(
    dataset: &[ExportRecord],
    page_index: usize,
    page_size: usize,
) -> PageResult {
    let offset = page_index * page_size;
    if offset >= dataset.len() {
        return PageResult {
            records: vec![],
            next_token: None,
        };
    }
    let end = std::cmp::min(offset + page_size, dataset.len());
    let records = dataset[offset..end].to_vec();
    let next_token = if end < dataset.len() {
        let last = &records[records.len() - 1];
        Some(CursorToken {
            last_seen_id: last.id,
            last_timestamp: last.created_at,
            page_index: page_index + 1,
        })
    } else {
        None
    };
    PageResult { records, next_token }
}

pub fn paginate_by_cursor(
    dataset: &[ExportRecord],
    token: Option<&CursorToken>,
    page_size: usize,
) -> PageResult {
    let start_pos = match token {
        None => 0,
        Some(cursor) => {
            let mut sorted = dataset.to_vec();
            sorted.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            sorted
                .iter()
                .position(|r| r.created_at > cursor.last_timestamp)
                .unwrap_or(sorted.len())
        }
    };

    if start_pos >= dataset.len() {
        return PageResult {
            records: vec![],
            next_token: None,
        };
    }

    let end = std::cmp::min(start_pos + page_size, dataset.len());
    let records = dataset[start_pos..end].to_vec();
    let page_idx = token.map(|t| t.page_index).unwrap_or(0);

    let next_token = if end < dataset.len() {
        let last = &records[records.len() - 1];
        Some(CursorToken {
            last_seen_id: last.id,
            last_timestamp: last.created_at,
            page_index: page_idx + 1,
        })
    } else {
        None
    };

    PageResult { records, next_token }
}

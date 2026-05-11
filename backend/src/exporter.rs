use anyhow::Result;
use crate::models::{
    CursorToken, ExportRecord, ExportRequest, ExportResult, ExportStatus, PaginationStrategy,
};
use crate::paginator;

pub fn export_page(
    dataset: &[ExportRecord],
    request: &ExportRequest,
    page_index: usize,
) -> paginator::PageResult {
    match request.strategy {
        PaginationStrategy::Offset => {
            paginator::paginate_by_page(dataset, page_index, request.page_size)
        }
        PaginationStrategy::Cursor => {
            let token = if page_index == 0 {
                None
            } else {
                Some(CursorToken {
                    last_seen_id: (page_index * request.page_size - 1) as u64,
                    last_timestamp: 0,
                    page_index,
                })
            };
            paginator::paginate_by_cursor(dataset, token.as_ref(), request.page_size)
        }
    }
}

pub fn export_all(
    dataset: &[ExportRecord],
    request: &ExportRequest,
) -> Result<ExportResult> {
    let mut all_records: Vec<ExportRecord> = Vec::new();
    let mut current_token: Option<CursorToken> = None;
    let mut pages_written = 0;

    loop {
        let result = match request.strategy {
            PaginationStrategy::Offset => {
                paginator::paginate_by_page(dataset, pages_written, request.page_size)
            }
            PaginationStrategy::Cursor => {
                paginator::paginate_by_cursor(dataset, current_token.as_ref(), request.page_size)
            }
        };

        if result.records.is_empty() {
            break;
        }

        all_records.extend(result.records);
        pages_written += 1;
        current_token = result.next_token;

        if current_token.is_none() {
            break;
        }
    }

    Ok(ExportResult {
        total_records: all_records.len(),
        pages_written,
    })
}

pub fn resume_export(
    dataset: &[ExportRecord],
    request: &ExportRequest,
    state_path: &str,
) -> Result<ExportResult> {
    let state_data = std::fs::read_to_string(state_path)?;
    let status: ExportStatus = serde_json::from_str(&state_data)?;

    let resume_token = CursorToken {
        last_seen_id: status.cursor.last_seen_id,
        last_timestamp: status.cursor.last_timestamp,
        page_index: 0,
    };

    let mut all_records: Vec<ExportRecord> = Vec::new();
    let mut current_token: Option<CursorToken> = Some(resume_token);
    let mut pages_written = status.pages_written;

    loop {
        let result = match request.strategy {
            PaginationStrategy::Offset => {
                paginator::paginate_by_page(dataset, current_token.as_ref().unwrap().page_index, request.page_size)
            }
            PaginationStrategy::Cursor => {
                paginator::paginate_by_cursor(dataset, current_token.as_ref(), request.page_size)
            }
        };

        if result.records.is_empty() {
            break;
        }

        all_records.extend(result.records);
        pages_written += 1;
        current_token = result.next_token;

        if current_token.is_none() {
            break;
        }
    }

    Ok(ExportResult {
        total_records: all_records.len(),
        pages_written,
    })
}

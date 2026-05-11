use export_retry_backend::models::*;
use export_retry_backend::paginator::*;
use export_retry_backend::exporter::*;

fn make_dataset(count: usize) -> Vec<ExportRecord> {
    generate_sample_dataset(count)
}

#[test]
fn test_page_based_no_drift() {
    let mut dataset = make_dataset(30);
    let page_size = 10;

    let result0 = paginate_by_page(&dataset, 0, page_size);
    let ids_page0: Vec<u64> = result0.records.iter().map(|r| r.id).collect();

    dataset.insert(0, ExportRecord {
        id: 999,
        entity_name: "inserted_during_export".into(),
        amount: 0.0,
        status: RecordStatus::Active,
        owner: "system".into(),
        created_at: 1700000000,
    });

    let result1 = paginate_by_page(&dataset, 1, page_size);
    let ids_page1: Vec<u64> = result1.records.iter().map(|r| r.id).collect();

    let duplicates: Vec<u64> = ids_page0
        .iter()
        .filter(|id| ids_page1.contains(id))
        .copied()
        .collect();

    assert!(
        duplicates.is_empty(),
        "offset pagination must not produce duplicates after insert, found: {:?}",
        duplicates
    );
}

#[test]
fn test_cursor_pagination_order() {
    let dataset: Vec<ExportRecord> = (0..25)
        .map(|i| ExportRecord {
            id: i as u64,
            entity_name: format!("rec_{}", i),
            amount: 100.0,
            status: RecordStatus::Active,
            owner: "owner".into(),
            created_at: 1700000000 + (i as u64 / 8),
        })
        .collect();

    let page_size = 8;
    let result0 = paginate_by_cursor(&dataset, None, page_size);
    assert_eq!(result0.records.len(), 8);

    let result1 = paginate_by_cursor(&dataset, result0.next_token.as_ref(), page_size);
    let result2 = paginate_by_cursor(&dataset, result1.next_token.as_ref(), page_size);

    let mut all_ids: Vec<u64> = Vec::new();
    all_ids.extend(result0.records.iter().map(|r| r.id));
    all_ids.extend(result1.records.iter().map(|r| r.id));
    all_ids.extend(result2.records.iter().map(|r| r.id));

    all_ids.sort();
    all_ids.dedup();

    assert_eq!(
        all_ids.len(),
        25,
        "cursor pagination must include all 25 records without skipping, got {}",
        all_ids.len()
    );
}

#[test]
fn test_resume_from_failure() {
    let dataset = make_dataset(50);
    let page_size = 10;

    let r0 = paginate_by_page(&dataset, 0, page_size);
    let r1 = paginate_by_page(&dataset, 1, page_size);
    let r2 = paginate_by_page(&dataset, 2, page_size);

    let saved_state = ExportStatus {
        pages_written: 3,
        total_records: 30,
        cursor: CursorToken {
            last_seen_id: r2.records.last().unwrap().id,
            last_timestamp: r2.records.last().unwrap().created_at,
            page_index: 3,
        },
        completed: false,
    };

    let state_path = std::env::temp_dir().join("export_state_test.json");
    std::fs::write(&state_path, serde_json::to_string(&saved_state).unwrap()).unwrap();

    let request = ExportRequest {
        page_size,
        strategy: PaginationStrategy::Offset,
        output_path: "test_output.json".into(),
    };

    let result = resume_export(&dataset, &request, state_path.to_str().unwrap()).unwrap();

    let _already_exported: Vec<u64> = r0.records.iter()
        .chain(r1.records.iter())
        .chain(r2.records.iter())
        .map(|r| r.id)
        .collect();

    assert_eq!(
        result.total_records, 20,
        "resume should export only remaining 20 records (pages 3-4), got {}",
        result.total_records
    );
}

#[test]
fn test_basic_export() {
    let dataset = make_dataset(15);
    let request = ExportRequest {
        page_size: 20,
        strategy: PaginationStrategy::Offset,
        output_path: "basic.json".into(),
    };

    let result = export_all(&dataset, &request).unwrap();
    assert_eq!(result.total_records, 15);
    assert_eq!(result.pages_written, 1);
}

#[test]
fn test_export_empty_dataset() {
    let dataset: Vec<ExportRecord> = vec![];
    let request = ExportRequest {
        page_size: 10,
        strategy: PaginationStrategy::Cursor,
        output_path: "empty.json".into(),
    };

    let result = export_all(&dataset, &request).unwrap();
    assert_eq!(result.total_records, 0);
    assert_eq!(result.pages_written, 0);
}

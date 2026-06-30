const CHARS_PER_TOKEN: usize = 4;

#[derive(Clone)]
struct MemoryEntry {
    content: String,
    importance: u8,
    timestamp: u64,
}

fn prune(entries: &mut Vec<MemoryEntry>, max_tokens: usize) {
    let max_chars = max_tokens * CHARS_PER_TOKEN;
    let total_chars: usize = entries.iter().map(|e| e.content.len()).sum();
    if total_chars <= max_chars {
        return;
    }
    entries.sort_by(|a, b| {
        a.importance
            .cmp(&b.importance)
            .then(a.timestamp.cmp(&b.timestamp))
    });
    while !entries.is_empty() {
        let total: usize = entries.iter().map(|e| e.content.len()).sum();
        if total <= max_chars {
            break;
        }
        entries.remove(0);
    }
    entries.sort_by_key(|e| e.timestamp);
}

#[test]
fn no_prune_when_under_budget() {
    let mut entries = vec![MemoryEntry {
        content: "short".to_string(),
        importance: 5,
        timestamp: 1,
    }];
    prune(&mut entries, 8000);
    assert_eq!(entries.len(), 1);
}

#[test]
fn prunes_lowest_importance_first() {
    let mut entries = vec![
        MemoryEntry { content: "a".repeat(100), importance: 1, timestamp: 1 },
        MemoryEntry { content: "b".repeat(100), importance: 9, timestamp: 2 },
        MemoryEntry { content: "c".repeat(100), importance: 5, timestamp: 3 },
    ];
    // Budget: 60 tokens = 240 chars. Total = 300. Must remove one.
    prune(&mut entries, 60);
    assert_eq!(entries.len(), 2);
    // Lowest importance (1) should be removed
    assert!(entries.iter().all(|e| e.importance > 1));
}

#[test]
fn respects_8k_token_limit() {
    let mut entries: Vec<MemoryEntry> = (0..20)
        .map(|i| MemoryEntry {
            content: "x".repeat(2000),
            importance: 5,
            timestamp: i,
        })
        .collect();
    // 20 entries × 2000 chars = 40000 chars = 10000 tokens. Limit is 8000.
    prune(&mut entries, 8000);
    let total_chars: usize = entries.iter().map(|e| e.content.len()).sum();
    assert!(total_chars <= 8000 * CHARS_PER_TOKEN);
}

#[test]
fn empty_entries_no_panic() {
    let mut entries: Vec<MemoryEntry> = Vec::new();
    prune(&mut entries, 8000);
    assert!(entries.is_empty());
}

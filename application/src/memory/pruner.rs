use super::store::MemoryEntry;

const CHARS_PER_TOKEN: usize = 4;

pub struct MemoryPruner;

impl MemoryPruner {
    pub fn prune(entries: &mut Vec<MemoryEntry>, max_tokens: usize) {
        let max_chars = max_tokens * CHARS_PER_TOKEN;

        let total_chars: usize = entries.iter().map(|e| e.content.len()).sum();
        if total_chars <= max_chars {
            return;
        }

        // Sort by importance (ascending), then by timestamp (oldest first)
        entries.sort_by(|a, b| {
            a.importance
                .cmp(&b.importance)
                .then(a.timestamp.cmp(&b.timestamp))
        });

        // Remove lowest-importance entries until under budget
        while !entries.is_empty() {
            let total: usize = entries.iter().map(|e| e.content.len()).sum();
            if total <= max_chars {
                break;
            }
            entries.remove(0);
        }

        // Re-sort by timestamp (chronological)
        entries.sort_by_key(|e| e.timestamp);
    }

    pub fn estimate_tokens(text: &str) -> usize {
        text.len() / CHARS_PER_TOKEN
    }
}

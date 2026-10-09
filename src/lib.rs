//! Append-only in-memory evidence records for agent research.
//! Persistent storage and cryptographic provenance are future work.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryRecord {
    pub id: u64,
    pub author: String,
    pub source: String,
    pub content: String,
}

#[derive(Default)]
pub struct Memory {
    records: Vec<MemoryRecord>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    MissingAuthor,
    MissingSource,
    EmptyContent,
}

impl Memory {
    pub fn append(
        &mut self,
        author: &str,
        source: &str,
        content: &str,
    ) -> Result<u64, MemoryError> {
        if author.trim().is_empty() {
            return Err(MemoryError::MissingAuthor);
        }
        if source.trim().is_empty() {
            return Err(MemoryError::MissingSource);
        }
        if content.trim().is_empty() {
            return Err(MemoryError::EmptyContent);
        }
        let id = self.records.len() as u64 + 1;
        self.records.push(MemoryRecord {
            id,
            author: author.into(),
            source: source.into(),
            content: content.into(),
        });
        Ok(id)
    }

    pub fn get(&self, id: u64) -> Option<&MemoryRecord> {
        self.records.iter().find(|record| record.id == id)
    }

    pub fn search(&self, query: &str) -> Vec<&MemoryRecord> {
        if query.trim().is_empty() {
            return Vec::new();
        }
        let query = query.to_lowercase();
        self.records
            .iter()
            .filter(|record| record.content.to_lowercase().contains(&query))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_and_retrieves_records() {
        let mut memory = Memory::default();
        let id = memory.append("agent-1", "experiment-1", "Bees forage").unwrap();
        assert_eq!(memory.get(id).unwrap().source, "experiment-1");
        assert_eq!(memory.len(), 1);
    }

    #[test]
    fn finds_case_insensitive_matches() {
        let mut memory = Memory::default();
        memory.append("agent-1", "note-1", "Bees forage").unwrap();
        assert_eq!(memory.search("BEES").len(), 1);
    }

    #[test]
    fn requires_provenance() {
        let mut memory = Memory::default();
        assert_eq!(
            memory.append("agent-1", "", "claim"),
            Err(MemoryError::MissingSource)
        );
        assert!(memory.is_empty());
    }

    #[test]
    fn returns_no_results_for_blank_query() {
        let mut memory = Memory::default();
        memory.append("agent-1", "note-1", "Bees forage").unwrap();
        assert!(memory.search(" ").is_empty());
    }
}

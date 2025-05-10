pub enum FileType {
    Csv,
    Excel, // todo
    Parquet,
    NdJson, 
}

impl FileType {
    pub fn determine_extension(path: &str) -> Option<Self> {
        if path.ends_with(".csv") {
            return Some(FileType::Csv);
        } else if path.ends_with(".xlsx") || path.ends_with(".xls") {
            return Some(FileType::Excel);
        } else if path.ends_with(".parquet") || path.ends_with(".pq") {
            return Some(FileType::Parquet);
        } else if path.ends_with(".ndjson") || path.ends_with(".jsonl") {
            return Some(FileType::NdJson);
        }
        None
    }
}
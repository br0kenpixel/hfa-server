use serde::Serialize;

#[derive(Serialize)]
pub struct ListResponse {
    pub files: Vec<FileMeta>,
}

#[derive(Serialize)]
pub struct FileMeta {
    pub name: String,
    pub size: u64,
    pub kind: FileType,
    pub modified: u64,
    pub accessed: u64,
    pub created: u64,
}

#[derive(Debug, Serialize)]
pub enum FileType {
    File,
    Directory,
}

impl From<std::fs::FileType> for FileType {
    fn from(kind: std::fs::FileType) -> Self {
        if kind.is_dir() {
            Self::Directory
        } else {
            Self::File
        }
    }
}

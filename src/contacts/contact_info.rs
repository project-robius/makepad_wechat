#[derive(Debug, Clone, PartialEq)]
pub enum ContactKind {
    People,
    FileTransfer,
    WeChat,
}

#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub name: String,
    pub kind: ContactKind,
}

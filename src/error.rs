use std::fmt;

#[derive(Debug)]
pub struct SmsgParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

impl SmsgParseError {
    #[allow(dead_code)]
    pub fn new(message: String, line: usize, col: usize) -> Self {
        Self { message, line, col }
    }

    #[allow(dead_code)]
    pub fn file_not_found(path: &str) -> Self {
        Self {
            message: format!("File not found: {}", path),
            line: 0,
            col: 0,
        }
    }

    #[allow(dead_code)]
    pub fn invalid_type(type_name: &str, line: usize, col: usize) -> Self {
        Self {
            message: format!("Invalid type: {}", type_name),
            line,
            col,
        }
    }

    pub fn duplicate_message(name: &str, line_num: usize) -> Self {
        Self {
            message: format!("Duplicate message definition: {}", name),
            line: line_num,
            col: 0,
        }
    }
}

impl fmt::Display for SmsgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line > 0 {
            write!(
                f,
                "Parse error at line {}, column {}: {}",
                self.line, self.col, self.message
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for SmsgParseError {}

#[derive(Debug)]
pub enum PackageError {
    TomlParse(String),
    MissingPackageSection,
    MissingField(String),
    InvalidEdition(String),
    FileNotFound(String),
    IoError(std::io::Error),
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageError::TomlParse(msg) => write!(f, "TOML parse error: {}", msg),
            PackageError::MissingPackageSection => write!(f, "Missing [package] section in package.toml"),
            PackageError::MissingField(field) => write!(f, "Missing required field: {}", field),
            PackageError::InvalidEdition(ed) => write!(f, "Invalid edition: {}. Expected '2026'", ed),
            PackageError::FileNotFound(path) => write!(f, "File not found: {}", path),
            PackageError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for PackageError {}

#[derive(Debug)]
pub enum ImportError {
    InvalidPackageName(String),
    MalformedSyntax(String),
    UnresolvableImport(String),
    IoError(std::io::Error),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::InvalidPackageName(name) => write!(f, "Invalid package name: {}", name),
            ImportError::MalformedSyntax(msg) => write!(f, "Malformed import syntax: {}", msg),
            ImportError::UnresolvableImport(imp) => write!(f, "Cannot resolve import: {}", imp),
            ImportError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ImportError {}

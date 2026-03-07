use crate::ir::{Field, FieldType, MessageDef};
use blake3::Hasher;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityStatus {
    Match,
    Mismatch,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MismatchDetail {
    pub message_name: String,
    pub hash1: [u8; 32],
    pub hash2: [u8; 32],
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct CompatibilityReport {
    pub status: CompatibilityStatus,
    pub details: Vec<MismatchDetail>,
}

pub fn compute_message_hash(message: &MessageDef) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hash_message(message, &mut hasher);
    *hasher.finalize().as_bytes()
}

fn hash_message(message: &MessageDef, hasher: &mut Hasher) {
    hasher.update(message.name.as_bytes());
    for field in &message.fields {
        hash_field(field, hasher);
    }
}

fn hash_field(field: &Field, hasher: &mut Hasher) {
    hasher.update(field.name.as_bytes());
    hash_field_type(&field.field_type, hasher);
}

fn hash_field_type(field_type: &FieldType, hasher: &mut Hasher) {
    match field_type {
        FieldType::Primitive(p) => {
            hasher.update(b"primitive:");
            hasher.update(p.to_string().as_bytes());
        }
        FieldType::Array(inner, size) => {
            hasher.update(b"array:");
            hash_field_type(inner, hasher);
            if let Some(s) = size {
                hasher.update(format!(":{}", s).as_bytes());
            }
        }
        FieldType::Nested(name) => {
            hasher.update(b"nested:");
            hasher.update(name.as_bytes());
        }
    }
}

#[allow(dead_code)]
pub fn compare_hashes(hash1: &[u8; 32], hash2: &[u8; 32]) -> bool {
    hash1 == hash2
}

#[allow(dead_code)]
pub fn compare_messages(
    messages1: &[(&str, [u8; 32])],
    messages2: &[(&str, [u8; 32])],
) -> CompatibilityReport {
    let mut details = Vec::new();
    
    let map1: std::collections::HashMap<&str, [u8; 32]> = 
        messages1.iter().map(|(k, v)| (*k, *v)).collect();
    let map2: std::collections::HashMap<&str, [u8; 32]> = 
        messages2.iter().map(|(k, v)| (*k, *v)).collect();
    
    for (name, hash1) in messages1 {
        if let Some(hash2) = map2.get(name) {
            if hash1 != hash2 {
                details.push(MismatchDetail {
                    message_name: name.to_string(),
                    hash1: *hash1,
                    hash2: *hash2,
                });
            }
        } else {
            details.push(MismatchDetail {
                message_name: name.to_string(),
                hash1: *hash1,
                hash2: [0u8; 32],
            });
        }
    }
    
    for (name, hash2) in messages2 {
        if !map1.contains_key(name) {
            details.push(MismatchDetail {
                message_name: name.to_string(),
                hash1: [0u8; 32],
                hash2: *hash2,
            });
        }
    }
    
    let status = if details.is_empty() {
        CompatibilityStatus::Match
    } else {
        CompatibilityStatus::Mismatch
    };
    
    CompatibilityReport { status, details }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Field, FieldType, PrimitiveType};

    #[test]
    fn test_compute_message_hash_returns_32_bytes() {
        let msg = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash = compute_message_hash(&msg);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_different_messages_produce_different_hashes() {
        let msg1 = MessageDef {
            name: "MessageA".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "MessageB".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_field_addition_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![
                Field {
                    name: "field1".to_string(),
                    field_type: FieldType::Primitive(PrimitiveType::String),
                    line: 1,
                    col: 1,
                },
                Field {
                    name: "field2".to_string(),
                    field_type: FieldType::Primitive(PrimitiveType::Int32),
                    line: 2,
                    col: 1,
                },
            ],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_field_removal_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![
                Field {
                    name: "field1".to_string(),
                    field_type: FieldType::Primitive(PrimitiveType::String),
                    line: 1,
                    col: 1,
                },
                Field {
                    name: "field2".to_string(),
                    field_type: FieldType::Primitive(PrimitiveType::Int32),
                    line: 2,
                    col: 1,
                },
            ],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_field_type_change_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::Int32),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_field_name_change_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "fieldA".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "fieldB".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_array_type_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Array(Box::new(FieldType::Primitive(PrimitiveType::String)), None),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_nested_type_changes_hash() {
        let msg1 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Primitive(PrimitiveType::String),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let msg2 = MessageDef {
            name: "TestMessage".to_string(),
            fields: vec![Field {
                name: "field1".to_string(),
                field_type: FieldType::Nested("CustomType".to_string()),
                line: 1,
                col: 1,
            }],
            line: 1,
            col: 1,
        };
        let hash1 = compute_message_hash(&msg1);
        let hash2 = compute_message_hash(&msg2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_compare_hashes_identical() {
        let hash1: [u8; 32] = [0u8; 32];
        let hash2: [u8; 32] = [0u8; 32];
        assert!(compare_hashes(&hash1, &hash2));
    }

    #[test]
    fn test_compare_hashes_different() {
        let hash1: [u8; 32] = [0u8; 32];
        let hash2: [u8; 32] = [1u8; 32];
        assert!(!compare_hashes(&hash1, &hash2));
    }

    #[test]
    fn test_compare_messages_all_match() {
        let messages1 = vec![("Msg1", [0u8; 32]), ("Msg2", [1u8; 32])];
        let messages2 = vec![("Msg1", [0u8; 32]), ("Msg2", [1u8; 32])];
        let report = compare_messages(&messages1, &messages2);
        assert_eq!(report.status, CompatibilityStatus::Match);
        assert!(report.details.is_empty());
    }

    #[test]
    fn test_compare_messages_mismatch() {
        let messages1 = vec![("Msg1", [0u8; 32])];
        let messages2 = vec![("Msg1", [1u8; 32])];
        let report = compare_messages(&messages1, &messages2);
        assert_eq!(report.status, CompatibilityStatus::Mismatch);
        assert_eq!(report.details.len(), 1);
        assert_eq!(report.details[0].message_name, "Msg1");
    }

    #[test]
    fn test_compare_messages_different_count() {
        let messages1 = vec![("Msg1", [0u8; 32]), ("Msg2", [1u8; 32])];
        let messages2 = vec![("Msg1", [0u8; 32])];
        let report = compare_messages(&messages1, &messages2);
        assert_eq!(report.status, CompatibilityStatus::Mismatch);
        assert_eq!(report.details.len(), 1);
    }

    #[test]
    fn test_compare_messages_missing_in_both() {
        let messages1 = vec![("Msg1", [0u8; 32])];
        let messages2 = vec![("Msg2", [1u8; 32])];
        let report = compare_messages(&messages1, &messages2);
        assert_eq!(report.status, CompatibilityStatus::Mismatch);
        assert_eq!(report.details.len(), 2);
    }
}

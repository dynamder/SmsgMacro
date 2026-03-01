use crate::error::SmsgParseError;
use crate::ir::{Field, FieldType, MessageDef, PrimitiveType, SmsgFile};

pub fn parse_smsg(input: &str) -> Result<SmsgFile, SmsgParseError> {
    let trimmed = input.trim();
    let mut messages = Vec::new();
    let lines: Vec<(usize, &str)> = trimmed
        .lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .collect();
    let mut i = 0;
    let mut seen_names = std::collections::HashSet::new();

    while i < lines.len() {
        let (line_num, line) = lines[i];

        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if line.starts_with("message") {
            let rest = line.strip_prefix("message").ok_or_else(|| {
                SmsgParseError::new("Invalid message keyword".to_string(), line_num, 1)
            })?;
            let name_with_brace = rest.trim();

            let (name, brace_on_same_line) = if let Some(pos) = name_with_brace.find('{') {
                let n = name_with_brace[..pos].trim();
                (n.to_string(), true)
            } else {
                (name_with_brace.trim().to_string(), false)
            };

            if name.is_empty() {
                return Err(SmsgParseError::new(
                    "Missing message name".to_string(),
                    line_num,
                    1,
                ));
            }

            if seen_names.contains(&name) {
                return Err(SmsgParseError::duplicate_message(&name, line_num));
            }
            seen_names.insert(name.clone());

            let mut fields = Vec::new();
            let mut found_brace = brace_on_same_line;

            if !found_brace {
                i += 1;
                if i < lines.len() && lines[i].1 == "{" {
                    found_brace = true;
                }
            }

            if !found_brace {
                return Err(SmsgParseError::new("Expected '{'".to_string(), line_num, 1));
            }

            i += 1;

            while i < lines.len() && lines[i].1 != "}" {
                let (field_line, field_str) = lines[i];
                if field_str.is_empty() || field_str.starts_with('#') {
                    i += 1;
                    continue;
                }

                let parts: Vec<&str> = field_str.split_whitespace().collect();
                if parts.len() >= 2 {
                    let field_type_str = parts[0];
                    let field_name = parts[1];

                    let field_type = parse_field_type(field_type_str)?;

                    fields.push(Field {
                        name: field_name.to_string(),
                        field_type,
                        line: field_line,
                        col: 1,
                    });
                }
                i += 1;
            }

            if i >= lines.len() {
                return Err(SmsgParseError::new("Expected '}'".to_string(), line_num, 1));
            }
            i += 1;

            messages.push(MessageDef {
                name,
                fields,
                line: line_num,
                col: 1,
            });
        } else {
            i += 1;
        }
    }

    Ok(SmsgFile { messages })
}

fn parse_field_type(type_str: &str) -> Result<FieldType, SmsgParseError> {
    if let Some(arr_start) = type_str.find('[')
        && let Some(arr_end) = type_str.find(']')
    {
        let base_type = &type_str[..arr_start];
        let size_str = &type_str[arr_start + 1..arr_end];

        let size = if size_str.is_empty() {
            None
        } else {
            Some(
                size_str
                    .parse()
                    .map_err(|_| SmsgParseError::new("Invalid array size".to_string(), 1, 1))?,
            )
        };

        let inner = parse_field_type(base_type)?;
        return Ok(FieldType::Array(Box::new(inner), size));
    }

    if let Some(primitive) = PrimitiveType::from_str(type_str) {
        return Ok(FieldType::Primitive(primitive));
    }

    Ok(FieldType::Nested(type_str.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_message() {
        let input = r#"message ChatMessage {
    string sender
    string content
    int64 timestamp
}"#;
        let result = parse_smsg(input).unwrap();
        assert_eq!(result.messages.len(), 1);
        let msg = &result.messages[0];
        assert_eq!(msg.name, "ChatMessage");
        assert_eq!(msg.fields.len(), 3);
        assert_eq!(msg.fields[0].name, "sender");
    }

    #[test]
    fn test_parse_array_type() {
        let input = r#"message Test {
    float64[] values
    int32[3] point
}"#;
        let result = parse_smsg(input).unwrap();
        assert_eq!(result.messages.len(), 1);
    }

    #[test]
    fn test_parse_nested_type() {
        let input = r#"message Position {
    float64 x
    float64 y
}

message RobotState {
    string name
    Position position
}"#;
        let result = parse_smsg(input).unwrap();
        assert_eq!(result.messages.len(), 2);
    }
}

use crate::error::SmsgParseError;
use crate::ir::{Field, FieldType, MessageDef, PrimitiveType, SmsgFile};
use winnow::ascii::{alpha1, digit0, multispace0, multispace1};
use winnow::combinator::opt;
use winnow::error::{ErrMode, InputError};
use winnow::prelude::*;
use winnow::token::take_while;

type WinnowError<'a> = InputError<&'a str>;
type WinnowResult<'a, T> = Result<T, ErrMode<WinnowError<'a>>>;

pub fn parse_smsg(input: &str) -> Result<SmsgFile, SmsgParseError> {
    let trimmed = input.trim();
    let mut remaining = trimmed;
    let mut messages = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    while !remaining.is_empty() {
        if let Err(e) = multispace0::<&str, InputError<&str>>.parse_next(&mut remaining) {
            return Err(SmsgParseError::new(
                format!("Whitespace error: {:?}", e),
                1,
                1,
            ));
        }

        if remaining.is_empty() {
            break;
        }

        if remaining.starts_with('#') {
            let _: Result<_, ErrMode<InputError<&str>>> =
                take_while(0.., |c| c != '\n').parse_next(&mut remaining);
            continue;
        }

        match parse_message.parse_next(&mut remaining) {
            Ok(msg) => {
                if seen_names.contains(&msg.name) {
                    let line = remaining.lines().count();
                    return Err(SmsgParseError::duplicate_message(&msg.name, line));
                }
                seen_names.insert(msg.name.clone());
                messages.push(msg);
            }
            Err(e) => {
                let line = remaining.lines().count();
                return Err(SmsgParseError::new(
                    format!("Parse error: {:?}", e),
                    line.max(1),
                    1,
                ));
            }
        }
    }

    Ok(SmsgFile { messages })
}

fn parse_message<'a>(input: &mut &'a str) -> WinnowResult<'a, MessageDef> {
    "message".parse_next(input)?;
    multispace1.parse_next(input)?;
    let name = alpha1.parse_next(input)?;
    multispace0.parse_next(input)?;
    "{".parse_next(input)?;

    let mut fields = Vec::new();

    loop {
        multispace0.parse_next(input)?;

        if input.starts_with('}') {
            "}".parse_next(input)?;
            break;
        }

        if input.starts_with('#') {
            take_while(0.., |c| c != '\n').parse_next(input)?;
            continue;
        }

        let field = parse_field.parse_next(input)?;
        fields.push(field);
    }

    let line = input.lines().count();
    Ok(MessageDef {
        name: name.to_string(),
        fields,
        line,
        col: 1,
    })
}

fn parse_field<'a>(input: &mut &'a str) -> WinnowResult<'a, Field> {
    let type_str: String = if input.contains('[') {
        parse_array_type.parse_next(input)?
    } else {
        take_while(1.., |c| c != ' ' && c != '\t' && c != '\n')
            .map(|s: &str| s.to_string())
            .parse_next(input)?
    };

    multispace1.parse_next(input)?;
    let name = take_while(1.., |c| c != ' ' && c != '\t' && c != '\n').parse_next(input)?;
    multispace0.parse_next(input)?;
    opt(";".value(())).parse_next(input)?;
    multispace0.parse_next(input)?;

    let line = input.lines().count();
    let field_type =
        parse_field_type(&type_str).map_err(|_e| ErrMode::Backtrack(InputError::at(*input)))?;

    Ok(Field {
        name: name.to_string(),
        field_type,
        line,
        col: 1,
    })
}

fn parse_array_type<'a>(input: &mut &'a str) -> WinnowResult<'a, String> {
    let base_type =
        take_while(1.., |c| c != ' ' && c != '\t' && c != '\n' && c != '[').parse_next(input)?;
    "[".parse_next(input)?;
    let size: Option<&str> = opt(digit0).parse_next(input)?;
    "]".parse_next(input)?;

    let result = if let Some(s) = size {
        if s.is_empty() {
            format!("{}[]", base_type)
        } else {
            format!("{}[{}]", base_type, s)
        }
    } else {
        format!("{}[]", base_type)
    };

    Ok(result)
}

fn parse_field_type(type_str: &str) -> Result<FieldType, SmsgParseError> {
    if let Some(arr_start) = type_str.find('[') {
        let arr_end = type_str
            .find(']')
            .ok_or_else(|| SmsgParseError::new("Missing ']'".to_string(), 1, 1))?;
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

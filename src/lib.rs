use json_tools::{Buffer, BufferType, Lexer, TokenType};

pub fn fix(json: &str) -> String {
    let bytes = json.as_bytes();
    let mut fixed = Vec::with_capacity(bytes.len());
    let mut last_token_type = TokenType::Invalid;
    for token in Lexer::new(json.bytes(), BufferType::Bytes(32)) {
        match token.kind {
            TokenType::Number => {
                if let Buffer::MultiByte(bs) = token.buf {
                    if last_token_type == TokenType::CurlyOpen {
                        fixed.push(b'"');
                        fixed.extend(bs);
                        fixed.push(b'"');
                    } else if last_token_type == TokenType::Comma {
                        fixed.push(b'"');
                        fixed.extend(bs);
                        fixed.push(b'"');
                    } else {
                        fixed.extend(bs);
                    }
                }
            }
            TokenType::String => {
                if let Buffer::MultiByte(bs) = token.buf {
                    fixed.extend(bs);
                }
            }
            _ => {
                fixed.extend_from_slice(token.kind.as_ref().as_bytes());
            }
        }
        last_token_type = token.kind;
    }
    unsafe { String::from_utf8_unchecked(fixed) }
}

#[cfg(test)]
mod tests {
    use super::fix;

    #[test]
    fn test_fix_json() {
        let json = r#"{1: "foo", 2 : "bar"}"#;
        let fixed = fix(json);
        assert_eq!(fixed, r#"{"1":"foo","2":"bar"}"#);
    }
}

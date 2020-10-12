enum JsonType {
    NULL,
    TRUE,
    FALSE,
    // NUMBER,
    // STRING,
    // ARRAY,
    // OBJECT,
}

struct JsonValue {
    t: JsonType,
}

struct ParseContext<'a> {
    json_bytes: &'a [u8],
}

enum ParseState {
    PARSE_OK,
    PARSE_EXPECT_VALUE,
    PARSE_INVALID_VALUE,
    PARSE_ROOT_NOT_SINGULAR,
}

fn parse(v: &mut JsonValue, s: &String) -> ParseState {
    let mut context = ParseContext {
        json_bytes: s.as_bytes(),
    };
    parse_whitespace(&mut context);
    return parse_value(&mut context, v);
}

fn parse_whitespace(c: &mut ParseContext) {
    let mut idx = 0;
    let mut ch = c.json_bytes[idx];
    while ch == b' ' || ch == b'\n' || ch == b'\t' || ch == b'\r' {
        idx += 1;
        ch = c.json_bytes[idx];
    }
    c.json_bytes = &c.json_bytes[idx..];
}

fn parse_value(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
    match c.json_bytes[0] {
        b'n' => parse_literals(c, v, "null", JsonType::NULL),
        b't' => parse_literals(c, v, "true", JsonType::TRUE),
        b'f' => parse_literals(c, v, "false", JsonType::FALSE),
        b'"' => parse_string(c, v),
        b'\0' => ParseState::PARSE_EXPECT_VALUE,
        _ => ParseState::PARSE_INVALID_VALUE,
    }
}

fn parse_literals(c: &mut ParseContext, v: &mut JsonValue, s: &str, t: JsonType) -> ParseState {
    let bs = c.json_bytes;
    let mut last_idx = 0;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if bs[i] != item {
            return ParseState::PARSE_INVALID_VALUE;
        }
        last_idx = i;
    }
    c.json_bytes = &bs[last_idx + 1..];
    v.t = t;
    return ParseState::PARSE_OK;
}

fn parse_string(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
    let bs = c.json_bytes;
    assert!(bs[0] == b'"');
    let mut idx = 1;
    while bs[idx] != b'"' {
        idx += 1;
    }
    let new_bs = &bs[..idx+2];
    let s = String::from_utf8_lossy(new_bs);
    return ParseState::PARSE_OK;
}

// fn parse_null(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
//   let bs = c.json_bytes;
//   if bs[0] != b'n' || bs[1] != b'u' || bs[2] != b'l' || bs[3] != b'l' {
//     return ParseState::PARSE_INVALID_VALUE;
//   } else {
//     c.json_bytes = &c.json_bytes[4..];
//     v.t = JsonType::NULL;
//     return ParseState::PARSE_OK;
//   }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_ok(s: &str) {
        let test_json = String::from(s);
        let mut v = JsonValue { t: JsonType::NULL };
        let state = parse(&mut v, &test_json);
        assert!(matches!(state, ParseState::PARSE_OK));
    }

    #[test]
    fn test_parse_value() {
        test_parse_ok("null");
        test_parse_ok("true");
        test_parse_ok("false");
    }
}

fn main() {}

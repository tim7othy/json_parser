
enum JsonType {
  NULL,
  TRUE,
  FALSE,
  NUMBER,
  STRING,
  ARRAY,
  OBJECT,
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
  PARSE_ROOT_NOT_SINGULAR
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
    println!("({}): ({})", idx, c.json_bytes[idx].to_ascii_lowercase());
    idx += 1;
    ch = c.json_bytes[idx];
  }
  c.json_bytes = &c.json_bytes[idx..];
}

fn parse_value(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
  match c.json_bytes[0] {
    b'n' => parse_null(c, v),
    b'\0' => ParseState::PARSE_EXPECT_VALUE,
    _ => ParseState::PARSE_INVALID_VALUE,
  } 
}

fn parse_null(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
  let bs = c.json_bytes;
  if bs[0] != b'n' || bs[1] != b'u' || bs[2] != b'l' || bs[3] != b'l' {
    return ParseState::PARSE_INVALID_VALUE;
  } else {
    println!("(word): ({})", c.json_bytes[0].to_ascii_lowercase());
    c.json_bytes = &c.json_bytes[4..];
    return ParseState::PARSE_OK;
  }
}

fn main() {
  let test_json = String::from(" null ");
  let mut v = JsonValue {
    t: JsonType::NULL,
  };
  parse(&mut v, &test_json);
}

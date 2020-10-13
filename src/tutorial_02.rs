// JsonValue是将Json字符串解析成使用当前语言表示的一个内置的数据结构
// 这个数据结构中包含从Json中不同部分解析得到的不同类型的值
// 一开始我们使用一个JsonType的枚举类型作为不同值的标记（tag）
// 使用JsonValue struct来表示这个值，但是这存在一个问题
// 不同Json类型的值内部包含的数据类型不同
// 方法1 为每个JsonValue类型设计一个不同结构，缺点在于耦合性太高
//       每个类型的解析函数与类型值的struct耦合
// 方法2 面向对象的程序设计中，抽象出一个高级类型，包含所有JsonValue通用的部分，然后为
//       Array、String等不同类型分别设计子类型包含各自独特的数据
// 方法3 C语言中可以通过在struct中引入一个Union，这就可以在一结构j中附带任意的其它数结构j
// 方法4 使用rust中特有的enum可以附带数据以及pattern matching特性

// enum JsonType {
//     NULL,
//     TRUE,
//     FALSE,
//     // NUMBER,
//     // STRING,
//     // ARRAY,
//     // OBJECT,
// }

// struct JsonValue {
//     t: JsonType,
// }


// If you try and create a recursive enum in Rust without using Box, you will get a compile time error saying that the enum can't be sized.
//
// This gives an error!
// enum List {
//     Nil,
//     Cons(i32, List)
// }
// In order for the enum to have a defined size, the recursively contained value must be in a Box.
//
// This works!
// enum List {
//     Nil,
//     Cons(i32, Box<List>)
// }
// This works because Box always has the same size no matter what T is, which allows Rust to give List a size.

enum Json {
    NULL,
    TRUE,
    FALSE,
    // NUMBER,
    STRING(String),
    ARRAY(Box<Json>),
    // OBJECT,
}

struct ParseContext<'a> {
    json_bytes: &'a [u8],
}

enum ParseState {
    Ok,
    ExpectValue,
    InvalidValue,
    RootNotSingular,
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
        b'\0' => ParseState::ExpectValue,
        _ => ParseState::InvalidValue,
    }
}

fn parse_literals(c: &mut ParseContext, v: &mut JsonValue, s: &str, t: JsonType) -> ParseState {
    let bs = c.json_bytes;
    let mut last_idx = 0;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if bs[i] != item {
            return ParseState::InvalidValue;
        }
        last_idx = i;
    }
    c.json_bytes = &bs[last_idx + 1..];
    v.t = t;
    return ParseState::Ok;
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
    let sr = s.to_string();
    return ParseState::Ok;
}

// fn parse_null(c: &mut ParseContext, v: &mut JsonValue) -> ParseState {
//   let bs = c.json_bytes;
//   if bs[0] != b'n' || bs[1] != b'u' || bs[2] != b'l' || bs[3] != b'l' {
//     return ParseState::InvalidValue;
//   } else {
//     c.json_bytes = &c.json_bytes[4..];
//     v.t = JsonType::NULL;
//     return ParseState::Ok;
//   }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_ok(s: &str) {
        let test_json = String::from(s);
        let mut v = JsonValue { t: JsonType::NULL };
        let state = parse(&mut v, &test_json);
        assert!(matches!(state, ParseState::Ok));
    }

    #[test]
    fn test_parse_value() {
        test_parse_ok("null");
        test_parse_ok("true");
        test_parse_ok("false");
    }
}

fn main() {

}

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
    NUMBER(u32),
    STRING(String),
    ARRAY(Vec<Box<Json>>),
    OBJECT(Box<Json>),
}

enum ParseState {
    Ok,
    ExpectValue,
    InvalidValue,
    RootNotSingular,
}

struct ParseContext<T: Iterator<Item = char>> {
    ch: Option<char>,
    restChars: T,
    stack: Vec<Box<Json>>,
}

fn nextChar<T>(c: &mut ParseContext<T>)
  where T: Iterator<Item = char>
{
  let ch = c.restChars.next();
  match ch {
    Some(x) => c.ch = Some(x),
    None => c.ch = None,
  }
}

fn parse(v: &mut Option<Json>, s: &String) -> ParseState {
    let mut context = ParseContext {
        ch: None,
        restChars: s.chars(),
        stack: vec![],
    };
    parse_whitespace(&mut context);
    return parse_value(&mut context, v);
}

fn parse_whitespace<T>(c: &mut ParseContext<T>)
  where T: Iterator<Item = char>
{
    nextChar(c);
    while matches!(c.ch, Some(' ')) ||
          matches!(c.ch, Some('\n')) ||
          matches!(c.ch, Some('\r')) ||
          matches!(c.ch, Some('\t')) {
          nextChar(c);
    }
}

fn parse_value<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    match c.ch {
        Some('n') => parse_literals(c, v, "null"),
        Some('t') => parse_literals(c, v, "true"),
        Some('f') => parse_literals(c, v, "false"),
        Some('"') => parse_string(c, v),
        None => ParseState::ExpectValue,
        _ => ParseState::InvalidValue,
    }
}

fn  parse_literals<T>(c: &mut ParseContext<T>, v: &mut Option<Json>, s: &str) -> ParseState
  where T: Iterator<Item = char>
{
    let mut chars = s.chars();
    while let Some(x) = chars.next() {
      if let Some(y) = c.ch {
        if x != y {
          return ParseState::InvalidValue;
        }
        nextChar(c)
      } else {
        return ParseState::InvalidValue;
      }
    }
    *v = match s {
        "null" => Some(Json::NULL),
        "true" => Some(Json::TRUE),
        "false" => Some(Json::FALSE),
        _ => None,
    };
    return ParseState::Ok;
}

fn parse_string<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    let mut s = String::new();
    nextChar(c);
    while let Some(x) = c.ch {
      if x == '"' {break;}
      s.push(x);
      nextChar(c);
    }
    if let None = c.ch {
      return ParseState::InvalidValue;
    } else {
      *v = Some(Json::STRING(s));
      return ParseState::Ok;
    }
}

fn test_parse_ok(s: &str) {
    let test_json = String::from(s);
    let mut v: Option<Json> = None;
    let state = parse(&mut v, &test_json);
    assert!(matches!(state, ParseState::Ok));
    match v {
      Some(Json::STRING(x)) => println!("String: {}", x),
      Some(Json::NULL) => println!("null"),
      Some(Json::TRUE) => println!("true"),
      Some(Json::FALSE) => println!("false"),
      _ => println!("None"),
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn test_parse_ok(s: &str) {
//         let test_json = String::from(s);
//         let mut v: Option<Json> = None;
//         let state = parse(&mut v, &test_json);
//         assert!(matches!(state, ParseState::Ok));
//         match v {
//           Some(Json::STRING(s)) => println!("String: {}", s),
//           Some(Json::NULL) => println!("null"),
//           Some(Json::TRUE) => println!("true"),
//           Some(Json::FALSE) => println!("false"),
//           _ => println!("None"),
//         }
//     }

//     #[test]
//     fn test_parse_value() {
//         test_parse_ok("null");
//         test_parse_ok("true");
//         test_parse_ok("false");
//     }
// }

fn main() {
    test_parse_ok("null");
    test_parse_ok("true");
    test_parse_ok("false");
    test_parse_ok("\"hello world\"");
}

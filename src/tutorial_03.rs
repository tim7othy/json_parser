use std::collections::HashMap;
enum Json {
    NULL,
    TRUE,
    FALSE,
    NUMBER(u32),
    STRING(String),
    ARRAY(Vec<Box<Json>>),
    OBJECT(HashMap<String, Box<Json>>),
}

enum ParseState {
    Ok,
    ExpectValue,
    InvalidValue,
    // RootNotSingular,
}

struct ParseContext<T: Iterator<Item = char>> {
    ch: Option<char>,
    rest_chars: T,
    // stack: Vec<Box<Json>>,
}

fn next_char<T>(c: &mut ParseContext<T>)
  where T: Iterator<Item = char>
{
  let ch = c.rest_chars.next();
  match ch {
    Some(x) => c.ch = Some(x),
    None => c.ch = None,
  }
}

fn parse(v: &mut Option<Json>, s: &String) -> ParseState {
    let mut context = ParseContext {
        ch: None,
        rest_chars: s.chars(),
        // stack: vec![],
    };
    parse_whitespace(&mut context);
    return parse_value(&mut context, v);
}

fn parse_whitespace<T>(c: &mut ParseContext<T>)
  where T: Iterator<Item = char>
{
    next_char(c);
    while matches!(c.ch, Some(' ')) ||
          matches!(c.ch, Some('\n')) ||
          matches!(c.ch, Some('\r')) ||
          matches!(c.ch, Some('\t')) {
          next_char(c);
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
        Some('[') => parse_array(c, v),
        Some('{') => parse_object(c, v),
        None => ParseState::ExpectValue,
        Some(n) => {
            if n.is_digit(10) {
                return parse_number(c, v);
            } else {
                return ParseState::InvalidValue;
            }
        },
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
        next_char(c)
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


fn parse_number<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    let mut num = 0;
    while let Some(ch) = c.ch {
        if let Some(n) = ch.to_digit(10) {
            num = num * 10 + n;
            next_char(c);
        } else {
            break;
        }
    }
    *v = Some(Json::NUMBER(num));
    return ParseState::Ok;
}

fn parse_string<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    let mut s = String::new();
    next_char(c);
    while let Some(x) = c.ch {
      if x == '"' {break;}
      s.push(x);
      next_char(c);
    }
    if let None = c.ch {
      return ParseState::InvalidValue;
    } else {
      *v = Some(Json::STRING(s));
      return ParseState::Ok;
    }
}

fn parse_array<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    let mut arr: Vec<Box<Json>> = vec![];
    loop {
        let mut tmp_v: Option<Json> = None;
        //解析j一个数组元素
        parse_whitespace(c);
        let state = parse_value(c, &mut tmp_v);
        if !matches!(state, ParseState::Ok) {
            return state;
        }
        parse_whitespace(c);
        if let Some(j) = tmp_v {
            // print_json(&j);
            arr.push(Box::new(j));
        }
        if let Some(x) = c.ch {
            // print!("x: {}", x);
            if x == ']' {
                break;
            } else if x == ',' {
                continue;
            } else {
                return ParseState::InvalidValue;
            }
        }
    }
    *v = Some(Json::ARRAY(arr));
    return ParseState::Ok;
}

fn parse_object<T>(c: &mut ParseContext<T>, v: &mut Option<Json>) -> ParseState
  where T: Iterator<Item = char>
{
    let mut h: HashMap<String, Box<Json>> = HashMap::new();
    loop {
        let mut tmp_k: Option<Json> = None;
        let mut tmp_v: Option<Json> = None;
        parse_whitespace(c);
        parse_string(c, &mut tmp_k);
        parse_whitespace(c);
        if let Some(x) = c.ch {
            // print!("x: {}", x);
            if x != ':' {
                return ParseState::InvalidValue;
            }
        }
        parse_whitespace(c);
        parse_value(c, &mut tmp_v);
        parse_whitespace(c);
        if tmp_k.is_some() && tmp_v.is_some() {
            if let Some(Json::STRING(x)) = tmp_k {
                if let Some(y) = tmp_v {
                    h.insert(x, Box::new(y));
                }
            }
        } else {
            return ParseState::InvalidValue;
        }
        if let Some(x) = c.ch {
            if x == ',' {
                continue;
            } else if x == '}' {
                break;
            } else {
                return ParseState::InvalidValue;
            }
        }
    }
    *v = Some(Json::OBJECT(h));
    return ParseState::Ok;
}

fn test_parse_ok(s: &str) {
    let test_json = String::from(s);
    let mut v: Option<Json> = None;
    let state = parse(&mut v, &test_json);
    print_state(&state);
    if let Some(x) = v {
        print_json(&x);
    }
}

fn print_json(json: &Json) {
    match json {
      Json::NULL => println!("null"),
      Json::TRUE => println!("true"),
      Json::FALSE => println!("false"),
      Json::STRING(x) => println!("String: {}", x),
      Json::NUMBER(n) => println!("number: {}", n),
      Json::ARRAY(arr) => {
          println!("[");
          for r in arr {
              // 此处相当于 &**r
            print_json(r);
            println!(",");
          }
          println!("]");
      },
      Json::OBJECT(h) => {
          println!("{{");
          for (k, v) in h.iter() {
              print!("{}: ", k);
              print_json(v);
          }
          println!("}}");
      }
    //   _ => println!("none"),
    }
}

fn print_state(state: &ParseState) {
    match state {
        ParseState::Ok => println!("State: Ok"),
        ParseState::ExpectValue => println!("State: ExpectValue"),
        // ParseState::RootNotSingular => println!("State: RootNotSingular"),
        ParseState::InvalidValue => println!("State: InvalidValue"),
        // _ => println!("state: None"),
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
    // test_parse_ok("null");
    // test_parse_ok("true");
    // test_parse_ok("false");
    // test_parse_ok("\"hello world\"");
    // test_parse_ok("[ null , true , false ]");
    // test_parse_ok("{ \"a\" : { \"b\" : true } , \"c\" : false }");
    test_parse_ok("123456");
}

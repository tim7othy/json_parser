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

fn parse(s: &String) -> Result<Json, &'static str> {
    let mut context = ParseContext {
        ch: None,
        rest_chars: s.chars(),
    };
    next_char(&mut context);
    parse_whitespace(&mut context);
    return parse_value(&mut context);
}

fn parse_whitespace<T>(c: &mut ParseContext<T>)
  where T: Iterator<Item = char>
{
    while matches!(c.ch, Some(' ')) ||
          matches!(c.ch, Some('\n')) ||
          matches!(c.ch, Some('\r')) ||
          matches!(c.ch, Some('\t')) {
          next_char(c);
    }
}

fn parse_value<T>(c: &mut ParseContext<T>) -> Result<Json, &'static str>
  where T: Iterator<Item = char>
{
    match c.ch {
        Some('n') => parse_literals(c, "null"),
        Some('t') => parse_literals(c, "true"),
        Some('f') => parse_literals(c, "false"),
        Some('"') => parse_string(c),
        Some('[') => parse_array(c),
        Some('{') => parse_object(c),
        Some(n) => {
            if n.is_digit(10) {
                return parse_number(c);
            } else {
                return Err("parse value error");
            }
        },
        None => return Err("expect value error"),
    }
}

fn  parse_literals<T>(c: &mut ParseContext<T>, s: &str) -> Result<Json, &'static str>
  where T: Iterator<Item = char>
{
    let mut chars = s.chars();
    while let Some(x) = chars.next() {
      match c.ch {
        Some(y) => {
          if x != y {
            return Err("parse invalid literal");
          }
          next_char(c);
        },
        None => return Err("parse invalid literal"),
      }
    }
    return match s {
        "null" => Ok(Json::NULL),
        "true" => Ok(Json::TRUE),
        "false" => Ok(Json::FALSE),
        _ => Err("parse unknown literal"),
    };
}

fn parse_number<T>(c: &mut ParseContext<T>) -> Result<Json, &'static str>
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
    return Ok(Json::NUMBER(num));
}

fn parse_string<T>(c: &mut ParseContext<T>) -> Result<Json, &'static str>
  where T: Iterator<Item = char>
{
    let mut s = String::new();
    next_char(c);
    while let Some(x) = c.ch {
      if x == '"' {
        next_char(c);
        break;
      }
      s.push(x);
      next_char(c);
    }
    return Ok(Json::STRING(s));
}

fn parse_array<T>(c: &mut ParseContext<T>) -> Result<Json, &'static str>
  where T: Iterator<Item = char>
{
    let mut arr: Vec<Box<Json>> = vec![];
    next_char(c);
    loop {
        parse_whitespace(c);
        let res = parse_value(c);
        match res {
          Ok(x) => {
            arr.push(Box::new(x));
            parse_whitespace(c);
            match c.ch {
              Some(']') => {
                next_char(c);
                break;
              },
              Some(',') => {
                next_char(c);
                continue;
              },
              _ => return Err("parse invalid array"),
            }
          },
          Err(e) => return Err(e),
        }
    }
    return Ok(Json::ARRAY(arr));
}

fn parse_object<T>(c: &mut ParseContext<T>) -> Result<Json, &'static str>
  where T: Iterator<Item = char>
{
    let mut h: HashMap<String, Box<Json>> = HashMap::new();
    next_char(c);
    loop {
        parse_whitespace(c);
        let k = parse_string(c)?;
        parse_whitespace(c);
        match c.ch {
          Some(x) => {
            if x == ':' {
              next_char(c);
              parse_whitespace(c);
              let v = parse_value(c)?;
              if let Json::STRING(x) = k {
                h.insert(x, Box::new(v));
              } else {
                return Err("parse invalid key in object");
              }
              parse_whitespace(c);
              match c.ch {
                Some(',') => {
                  next_char(c);
                  continue;
                },
                Some('}') => {
                  next_char(c);
                  break;
                }
                None => return Err("parse invalid object"),
                _ => return Err("parse invalid object"),
              }
            } else {
              return Err("parse invalid object");
            }
          },
          None => return Err("parse invalid object"),
        }
    }
    return Ok(Json::OBJECT(h));
}

fn test_parse_ok(s: &str) {
    let test_json = String::from(s);
    let res = parse(&test_json);
    match res {
      Ok(x) => print_json(&x),
      Err(e) => println!("{}", e),
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
    test_parse_ok("[ null , true , false ]");
    test_parse_ok("{ \"a\" : { \"b\" : true } , \"c\" : false }");
    test_parse_ok("123456");
}

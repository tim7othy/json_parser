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

struct Parser<T: Iterator<Item = char>> {
    ch: Option<char>,
    rest_chars: T,
    // stack: Vec<Box<Json>>,
}

impl<T> Parser<T>
    where T: Iterator<Item = char>
{
    pub fn new(input: T) -> Self {
        let mut parser = Parser {
            ch: None,
            rest_chars: input,
        };
        parser.next_char();
        parser
    }

    fn next_char(&mut self) {
        let ch = self.rest_chars.next();
        match ch {
            Some(x) => self.ch = Some(x),
            None => self.ch = None,
        }
    }

    fn parse_whitespace(&mut self)
    {
        while matches!(self.ch, Some(' '))
            || matches!(self.ch, Some('\n'))
            || matches!(self.ch, Some('\r'))
            || matches!(self.ch, Some('\t'))
        {
            self.next_char();
        }
    }

    fn parse_value(&mut self) -> Result<Json, &'static str>
    {
        match self.ch {
            Some('n') => self.parse_literals("null"),
            Some('t') => self.parse_literals("true"),
            Some('f') => self.parse_literals("false"),
            Some('"') => self.parse_string(),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some(n) => {
                if n.is_digit(10) {
                    return self.parse_number();
                } else {
                    return Err("parse value error");
                }
            }
            None => return Err("expect value error"),
        }
    }

    fn parse_literals(&mut self, s: &str) -> Result<Json, &'static str>
    {
        let mut chars = s.chars();
        while let Some(x) = chars.next() {
            match self.ch {
                Some(y) => {
                    if x != y {
                        return Err("parse invalid literal");
                    }
                    self.next_char();
                }
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

    fn parse_number(&mut self) -> Result<Json, &'static str>
    {
        let mut num = 0;
        while let Some(ch) = self.ch {
            if let Some(n) = ch.to_digit(10) {
                num = num * 10 + n;
                self.next_char();
            } else {
                break;
            }
        }
        return Ok(Json::NUMBER(num));
    }

    fn parse_string(&mut self) -> Result<Json, &'static str>
    {
        let mut s = String::new();
        self.next_char();
        while let Some(x) = self.ch {
            if x == '"' {
                self.next_char();
                break;
            }
            s.push(x);
            self.next_char();
        }
        return Ok(Json::STRING(s));
    }

    fn parse_array(&mut self) -> Result<Json, &'static str>
    {
        let mut arr: Vec<Box<Json>> = vec![];
        self.next_char();
        loop {
            self.parse_whitespace();
            let res = self.parse_value();
            match res {
                Ok(x) => {
                    arr.push(Box::new(x));
                    self.parse_whitespace();
                    match self.ch {
                        Some(']') => {
                            self.next_char();
                            break;
                        }
                        Some(',') => {
                            self.next_char();
                            continue;
                        }
                        _ => return Err("parse invalid array"),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        return Ok(Json::ARRAY(arr));
    }

    fn parse_object(&mut self) -> Result<Json, &'static str>
    {
        let mut h: HashMap<String, Box<Json>> = HashMap::new();
        self.next_char();
        loop {
            self.parse_whitespace();
            let k = self.parse_string()?;
            self.parse_whitespace();
            match self.ch {
                Some(x) => {
                    if x == ':' {
                        self.next_char();
                        self.parse_whitespace();
                        let v = self.parse_value()?;
                        if let Json::STRING(x) = k {
                            h.insert(x, Box::new(v));
                        } else {
                            return Err("parse invalid key in object");
                        }
                        self.parse_whitespace();
                        match self.ch {
                            Some(',') => {
                                self.next_char();
                                continue;
                            }
                            Some('}') => {
                                self.next_char();
                                break;
                            }
                            None => return Err("parse invalid object"),
                            _ => return Err("parse invalid object"),
                        }
                    } else {
                        return Err("parse invalid object");
                    }
                }
                None => return Err("parse invalid object"),
            }
        }
        return Ok(Json::OBJECT(h));
    }

}

fn parse(s: &String) -> Result<Json, &'static str> {
    let mut parser = Parser::new(s.chars());
    parser.parse_whitespace();
    return parser.parse_value();
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
        }
        Json::OBJECT(h) => {
            println!("{{");
            for (k, v) in h.iter() {
                print!("{}: ", k);
                print_json(v);
            }
            println!("}}");
        } //   _ => println!("none"),
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

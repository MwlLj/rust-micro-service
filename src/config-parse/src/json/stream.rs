use json::JsonValue;

enum Mode {
    Word,
    Output,
}

enum OutputMode {
    FindSymbol,
    FindWord,
    Array
}

enum ObjectMode {
    Object,
    Array
}

const key_output: &str = "output";
const key_input: &str = "input";
const symbol_lt_lt: &str = "<<";

struct CJsonStream {
}

impl CJsonStream {
    fn parse(&self, json: &str, rule: &str) -> String {
        let jsonValue = match json::parse(json) {
            Ok(j) => j,
            Err(err) => {
                println!("json parse error, err: {}", err);
                return String::new();
            }
        };
        self.stream(rule, &mut |content: &str| -> Option<String> {
            if content.contains("\"") {
                let x: &[_] = &['"'];
                return Some(content.trim_matches(x).to_string());
            } else {
                let mut word = String::new();
                let mut indexStr = String::new();
                let bytes = content.bytes();
                let bytesLen = bytes.len();
                let mut index = 0;
                let mut value = &jsonValue;
                let mut result = String::new();
                let mut objectMode = ObjectMode::Object;
                for (i, b) in bytes.enumerate() {
                    if b == b'.' || i == bytesLen - 1 {
                        if i == bytesLen - 1 {
                            word.push(char::from(b));
                        }
                        // let w = word.trim();
                        let w = &word;
                        if w.len() == 0 {
                            continue;
                        }
                        if w == key_input {
                        } else {
                            // println!("w: {}, value: {:?}", w, value);
                            value = &value[w];
                            // println!("value: {:?}", value);
                            value = self.append(value, w, index, &mut result);
                        }
                        word.clear();
                    } else if b == b'[' {
                        objectMode = ObjectMode::Array;
                    } else if b == b']' {
                        index = match indexStr.parse::<usize>() {
                            Ok(v) => v,
                            Err(err) => {
                                println!("array index is not number");
                                0
                            }
                        };
                        // let w = word.trim();
                        let w = &word;
                        value = &value[w];
                        // println!("w: {:?}, value: {:?}, index: {}", w, value, index);
                        value = self.append(value, w, index, &mut result);
                        indexStr.clear();
                        word.clear();
                        objectMode = ObjectMode::Object;
                    } else {
                        let c = char::from(b);
                        match objectMode {
                            ObjectMode::Array => {
                                indexStr.push(c);
                            },
                            _ => {
                                word.push(c);
                            }
                        }
                    }
                }
                return Some(result);
            }
            println!("content: {}", content);
            None
        }, &mut |content: &str, index: u32| -> Option<String> {
            println!("content: {}, index: {}", content, index);
            None
        })
    }

    fn append<'b>(&self, jsonValue: &'b JsonValue, name: &str, index: usize, result: &mut String) -> &'b JsonValue {
        match jsonValue {
            JsonValue::Null => {
            },
            JsonValue::Short(v) => {
                result.push_str(&v.to_string());
                // println!("short: {}, result: {}", v, result);
            },
            JsonValue::String(v) => {
                result.push_str(&v);
            },
            JsonValue::Number(v) => {
                result.push_str(&v.to_string());
            },
            JsonValue::Boolean(v) => {
                result.push_str(&v.to_string());
            },
            JsonValue::Object(v) => {
                if let Some(v) = v.get(name) {
                    return v;
                };
            },
            JsonValue::Array(v) => {
                // println!("before: {:?}, after: {:?}, filed: {:?}", v, v[index], v[index]["field"]);
                return &(v[index]);
            }
        }
        jsonValue
    }

    fn stream<Object, Array>(&self, rule: &str, objFn: &mut Object, arrFn: &mut Array) -> String
        where Object: FnMut(&str) -> Option<String>
        , Array: FnMut(&str, u32) -> Option<String> {
        let bytes = rule.bytes();
        let mut outputContent = String::new();
        let mut mode = Mode::Word;
        let mut outputMode = OutputMode::FindSymbol;
        let mut word = String::new();
        let mut arrNumber = String::new();
        for b in bytes {
            match mode {
                Mode::Word => {
                    if b == b' ' || b == b'\t' || b == b'\n' {
                        let w = word.trim();
                        if w == key_output {
                            mode = Mode::Output;
                            outputMode = OutputMode::FindSymbol;
                        }
                        word.clear();
                        continue;
                    } else {
                        word.push(char::from(b));
                    }
                },
                Mode::Output => {
                    if b == b' ' || b == b'\t' || b == b'\n' {
                        let w = word.trim();
                        if w.len() == 0 {
                            continue;
                        }
                        if w == symbol_lt_lt {
                            outputMode = OutputMode::FindWord;
                            word.clear();
                            continue;
                        } else {
                            match outputMode {
                                OutputMode::Array => {
                                },
                                OutputMode::FindWord => {
                                    // is object, not array
                                    if let Some(s) = objFn(&w) {
                                        outputContent.push_str(&s);
                                    };
                                },
                                _ => {
                                }
                            }
                            outputMode = OutputMode::FindSymbol;
                        }
                        word.clear();
                        continue;
                    /*
                    } else if b == b'[' {
                        outputMode = OutputMode::Array;
                        arrNumber.clear();
                        continue;
                    */
                    } else if b == b';' {
                        match outputMode {
                            OutputMode::Array => {
                            },
                            OutputMode::FindWord => {
                                // is object, not array
                                if let Some(s) = objFn(&word.trim()) {
                                    outputContent.push_str(&s);
                                };
                            },
                            _ => {
                            }
                        }
                        mode = Mode::Word;
                        word.clear();
                        continue;
                    } else {
                        match outputMode {
                            OutputMode::Array => {
                            },
                            _ => {
                                word.push(char::from(b));
                            }
                        }
                    }
                    /*
                    match outputMode {
                        OutputMode::FindSymbol => {
                        },
                        OutputMode::FindWord => {
                        },
                        OutputMode::Array => {
                            if b == b']' {
                                match arrNumber.parse::<u32>() {
                                    Ok(index) => {
                                        if let Some(s) = arrFn(&word.trim(), index) {
                                            outputContent.push_str(&s);
                                        };
                                    },
                                    Err(err) => {
                                        println!("arr index is not number");
                                        continue;
                                    }
                                };
                                arrNumber.clear();
                                outputMode = OutputMode::FindSymbol;
                            } else {
                                arrNumber.push(char::from(b));
                            }
                        }
                    }
                    */
                }
            }
        }
        outputContent
    }
}

impl CJsonStream {
    fn new() -> CJsonStream {
        CJsonStream{}
    }
}

/// push json field to string
/// # params
/// * rule param
/// ```
/// output << input.name;
/// output << input.name << "." << input.home[0].field << "." << input.home[1].field << input.age << input.age << input.name;
/// output << input.home[0].field << input.age;
/// ```
/// # example
/// ```
/// let result = parse(r#"
///     {
///         "name": "jake",
///         "age": 20,
///         "home": [
///             {
///                 "field": 1
///             },
///             {
///                 "field": 2
///             }
///         ],
///         "like": [
///         ]
///     }
///         "#, r#"
///         output << input.name;
///         output << input.name << "." << input.home[0].field << "." << input.home[1].field << input.age << input.age << input.name;
///         output << input.home[0].field << input.age;
///     "#);
/// println!("{:?}", result);
/// assert_eq!("jakejake.1.22020jake120", result);
/// ```
pub fn parse(json: &str, rule: &str) -> String {
    CJsonStream::new().parse(json, rule)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // #[ignore]
    fn jsonStreamTest() {
        let result = CJsonStream::new().parse(r#"
        {
            "name": "jake",
            "age": 20,
            "home": [
                {
                    "field": 1
                },
                {
                    "field": 2
                }
            ],
            "like": [
            ]
        }
            "#, r#"
            output << input.name;
            output << input.name << "." << input.home[0].field << "." << input.home[1].field << input.age << input.age << input.name;
            output << input.home[0].field << input.age;
        "#);
        assert_eq!("jakejake.1.22020jake120", result);
        println!("{:?}", result);
    }

    /*
, r#"
            output << input.name;
            output << input.name << "." << input.home[0].field << "." << input.home[1].field << input.age << input.age << input.name;
            output << input.home[0].field << input.age;
        "#
    */

    #[test]
    #[ignore]
    fn jsonTest() {
        let value = match json::parse(r#"
        {
            "user": {
                "name": "jake",
                "age": 20
            }
        }
        "#) {
            Ok(v) => v,
            Err(err) => {
                assert!(false);
                return;
            }
        };
        let v = &JsonValue::Null;
        let v = &value["user"];
        println!("{:?}", v["name"]);
    }
}

enum Mode {
    Word,
    Array,
}

struct CJsonStream {
}

impl CJsonStream {
    fn parse(&self, json: &str, router: &str) -> String {
        String::new()
    }

    fn stream<Object, Array>(&self, router: &str, objFn: Object, arrFn: &mut Array)
        where Object: FnMut(&str) -> Option<String>
        , Array: FnMut(&str, u32) -> Option<String> {
        let bytes = router.bytes();
        let mut mode = Mode::Word;
        let mut word = String::new();
        let mut arrNumber = String::new();
        for b in bytes {
            match mode {
                Mode::Word => {
                    if b == b' ' || b == b'\t' || b == b'\n' {
                        let w = word.trim();
                        if w == "output" {
                        } else if w == "<<" {
                        }
                        word.clear();
                    } else if b == b'[' {
                        mode = Mode::Array;
                    }
                },
                Mode::Array => {
                    if b == b']' {
                        match arrNumber.parse::<u32>() {
                            Ok(index) => {
                                arrFn(&word, index);
                            },
                            Err(err) => {
                                println!("arr index is not number");
                                continue;
                            }
                        };
                        mode = Mode::Word;
                        arrNumber.clear();
                    } else {
                        arrNumber.push(char::from(b));
                    }
                }
            }
        }
    }
}

impl CJsonStream {
    fn new() -> CJsonStream {
        CJsonStream{}
    }
}

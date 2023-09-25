use std::collections::HashMap;
use std::fmt;

pub struct Flip<'a> {
    symbols: HashMap<&'a str, FlipType<'a>>
}

impl <'a> Flip<'a> {
    pub fn new() -> Flip<'a> {
        Flip {
            symbols: HashMap::new()
        }
    }

    fn literal(&self, ft: FlipType<'a>) -> FlipType<'a> {
        match ft {
            FlipType::Atom(name) => {
                if let Some(v) = self.symbols.get(name) {
                    v.clone()
                } else {
                    ft
                }
            },
            _ => ft
        }
    }

    pub fn call_fn(&mut self, name: &str, mut args: Vec<FlipType<'a>>) -> FlipType<'a> {
        let mut get_next_arg = || {
            if args.len() > 0 {
                self.literal(args.remove(0))
            } else {
                FlipType::None
            }
        };

        match name {
            "+" => {
                let a = get_next_arg();
                let b = get_next_arg();

                if let FlipType::Int(v1) = a {
                    if let FlipType::Int(v2) = b {
                        FlipType::Int(v1 + v2)
                    } else {
                        FlipType::None
                    }
                } else {
                    FlipType::None
                }
            }
            "print" => {
                loop {
                    let arg = get_next_arg();
                    if arg == FlipType::None { break; }
                    println!("{}", arg);
                }
                
                FlipType::None
            },
            "list" => {
                let car = get_next_arg();
                FlipType::List(Box::new(car), Box::new(FlipType::None))
            },
            "push" => {
                let car = get_next_arg();
                let cdr = get_next_arg();

                FlipType::List(Box::new(car), Box::new(cdr))
            },
            "car" => {
                let list = get_next_arg();

                if let FlipType::List(car, _) = list {
                    *car
                } else {
                    FlipType::None
                }
            }
            "cdr" => {
                let list = get_next_arg();

                if let FlipType::List(_, cdr) = list {
                    *cdr
                } else {
                    FlipType::None
                }
            }
            "let" => {
                let alias = get_next_arg();
                let value = get_next_arg();

                if let FlipType::Atom(alias_name) = alias {
                    self.symbols.insert(alias_name, value);
                    self.literal(alias)
                } else {
                    FlipType::None
                }
            }
            _ => FlipType::None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlipType<'a> {
    Atom(&'a str),
    List(Box<FlipType<'a>>, Box<FlipType<'a>>),
    Int(i32),
    String(&'a str),
    None,
}


impl fmt::Display for FlipType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FlipType::Atom(v) => write!(f, "{v}"),
            FlipType::String(v) => write!(f, "\"{v}\""),
            FlipType::Int(v) => write!(f, "{v}"),
            FlipType::None => write!(f, "()"),
            FlipType::List(_, _) => {
                let mut out = "(".to_string();
                let mut curr = self;

                loop {
                    if let FlipType::List(car, cdr) = curr {
                        if **cdr == FlipType::None {
                            out.push_str(&format!("{})", *car));
                            break;
                        } else {
                            out.push_str(&format!("{}, ", *car));
                            curr = cdr;
                        }
                    }
                }

                write!(f, "{out}")
            }
        }
    }
}

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum FlipType<'a> {
    Atom(&'a str),
    List(Box<FlipType<'a>>, Box<FlipType<'a>>),
    Func(Vec<FlipType<'a>>, Box<FlipType<'a>>),
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
            FlipType::Func(_, _) => write!(f, ""),
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

#[derive(Debug)]
pub struct Flip<'a> {
    symbols: HashMap<&'a str, Vec<FlipType<'a>>>,
    scope_level: usize
}

impl <'a> Flip<'a> {
    pub fn new() -> Flip<'a> {
        Flip {
            symbols: HashMap::new(),
            scope_level: 1
        }
    }

    fn get_symbol(&self, name: &'a str) -> FlipType<'a> {
        if let Some(v) = self.symbols.get(name) {
            v.last().unwrap().clone()
        } else {
            FlipType::Atom(name)
        }
    }

    fn insert_symbol(&mut self, name: &'a str, val: FlipType<'a>) {
        self.symbols.entry(name).or_insert(vec![FlipType::None; self.scope_level-1]).push(val);
    }

    fn increase_scope(&mut self) {
        self.scope_level += 1;
    }

    fn decrease_scope(&mut self) {
        let mut reduce_scope = false;
        for (_, val) in self.symbols.iter_mut() {
            if val.len() == self.scope_level {
                reduce_scope = true;
                val.pop();
            }
        }
        if reduce_scope {
            self.scope_level -= 1;
        }
    }

    fn literal(&self, ft: FlipType<'a>) -> FlipType<'a> {
        match ft {
            FlipType::Atom(name) => self.get_symbol(name),
            _ => ft
        }
    }

    fn list(car: FlipType<'a>, cdr: FlipType<'a>) -> FlipType<'a> {
        FlipType::List(Box::new(car), Box::new(cdr))
    }

    fn get_arg(&mut self, args: &Vec<FlipType<'a>>, i: usize) -> FlipType<'a> {
        let x = self.literal(self.literal((*args.get(i).unwrap_or(&FlipType::None)).clone()));
        self.eval(x)
    }

    pub fn call_fn(&mut self, name: &'a str, args: Vec<FlipType<'a>>) -> FlipType<'a> {
        match name {
            "if" => {
                let cond = self.get_arg(&args, 0);

                if cond != FlipType::None {
                    self.get_arg(&args, 1)
                } else {
                    self.get_arg(&args, 2)
                }
            }
            "<=" => {
                let a = self.get_arg(&args, 0);
                let b = self.get_arg(&args, 1);

                if let FlipType::Int(v1) = a {
                    if let FlipType::Int(v2) = b {
                        if v1 <= v2 {
                            return FlipType::Int(v1);
                        }
                    }
                }

                FlipType::None
            }
            "+" => {
                let a = self.get_arg(&args, 0);
                let b = self.get_arg(&args, 1);

                if let FlipType::Int(v1) = a {
                    if let FlipType::Int(v2) = b {
                        return FlipType::Int(v1 + v2);
                    }
                }

                FlipType::None
            }
            "-" => {
                let a = self.get_arg(&args, 0);
                let b = self.get_arg(&args, 1);

                if let FlipType::Int(v1) = a {
                    if let FlipType::Int(v2) = b {
                        return FlipType::Int(v1 - v2);
                    }
                }

                FlipType::None
            }
            "*" => {
                let a = self.get_arg(&args, 0);
                let b = self.get_arg(&args, 1);

                if let FlipType::Int(v1) = a {
                    if let FlipType::Int(v2) = b {
                        return FlipType::Int(v1 * v2);
                    }
                }

                FlipType::None
            }
            "print" => {
                let arg = self.get_arg(&args, 0);
                println!("{arg}");
                FlipType::None
            },
            "list" => {
                let car = self.get_arg(&args, 0);

                Self::list(car, FlipType::None)
            },
            "push" => {
                let car = self.get_arg(&args, 0);
                let cdr = self.get_arg(&args, 1);

                Self::list(car, cdr)
            },
            "car" => {
                let list = self.get_arg(&args, 0);

                if let FlipType::List(car, _) = list {
                    *car
                } else {
                    FlipType::None
                }
            }
            "cdr" => {
                let list = self.get_arg(&args, 0);

                if let FlipType::List(_, cdr) = list {
                    *cdr
                } else {
                    FlipType::None
                }
            }
            "let" => {
                let alias = self.get_arg(&args, 0);
                let value = self.get_arg(&args, 1);

                if let FlipType::Atom(alias_name) = alias {
                    self.insert_symbol(alias_name, value);
                    self.literal(alias)
                } else {
                    FlipType::None
                }
            }
            _ => {
                let func = self.get_symbol(name);

                if let FlipType::Func(func_args, statements) = func {
                    self.increase_scope();

                    for (arg_name, value) in func_args.iter().zip(args.into_iter()) {
                        if let FlipType::Atom(arg_name) = arg_name {
                            if let FlipType::Atom(atom_name) = value {
                                let value = self.get_symbol(atom_name);
                                let evaluated = self.eval(value);
                                self.insert_symbol(arg_name, evaluated);
                            } else {
                                let evaluated = self.eval(value);
                                self.insert_symbol(arg_name, evaluated);
                            }
                        }
                    }

                    if let FlipType::List(result, _) = self.eval(*statements) {
                        let return_value = (*Self::list_to_vec(*result).last().unwrap_or(&FlipType::None)).clone();
                        self.decrease_scope();
                        return return_value;
                    }
                }

                FlipType::None
            }
        }
    }

    fn eval(&mut self, v: FlipType<'a>) -> FlipType<'a> {
        if let FlipType::List(car, cdr) = v {
            if let FlipType::Atom(name) = *car {
                self.eval_fn(name, *cdr)
            } else {
                let car = self.eval(*car);
                let cdr = self.eval(*cdr);
                Self::list(car, cdr)
            }
        } else {
            v
        }
    }

    fn list_to_vec(list: FlipType<'a>) -> Vec<FlipType<'a>> {
        let mut curr = list;
        let mut vec = Vec::new();

        while let FlipType::List(car, cdr) = curr {
            vec.push(*car);
            curr = *cdr;
        }

        vec
    }

    fn eval_fn(&mut self, name: &'a str, args: FlipType<'a>) -> FlipType<'a> {
        if name == "fn" {
            if let FlipType::List(func_name, func_data) = args {
                if let FlipType::Atom(func_name) = *func_name {
                    if let FlipType::List(func_args, func_body) = *func_data {
                        let func_args = Self::list_to_vec(*func_args);
                        self.insert_symbol(func_name, FlipType::Func(func_args, func_body));
                        return FlipType::Atom(func_name)
                    }
                }
            }

            FlipType::None
        } else {
            let args_vec = Self::list_to_vec(args);
            self.call_fn(name, args_vec)
        }
    }
}

fn interpret(code: &[FlipType]) {
    let mut state = Flip::new();

    for list in code {
        if let FlipType::List(_, _) = list {
            state.eval(list.clone());
        }
    }
}

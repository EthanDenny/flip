const CODE: &str = "
(const true 1)
(const false 0)

(= a 1)
(print a)

(if (and (== a 1.2) (== true false))
    (print \"a is 1.2\")
    ()
)

(print (+ (+ 2 2) (* 2 2) (exp 2 2)))
";

struct Token<'a> {
    content: &'a str,
    line: usize,
}

fn main() {
    let mut chars = CODE.chars().enumerate().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 1;
    
    while let Some((i, c)) = chars.next() {
        match c {
            '(' | ')' => {
                let t = Token {
                    content: &CODE[i..i+1],
                    line: line,
                };

                tokens.push(t);
            }
            '\n' => {
                line += 1;
            }
            '"' => {
                while let Some(&(j, c)) = chars.peek() {
                    match c {
                        '"' => {
                            let t = Token {
                                content: &CODE[i..j+1],
                                line: line,
                            };

                            tokens.push(t);

                            chars.next();
                            break;
                        }
                        _ => {
                            chars.next();
                        }
                    }
                }
            }
            '0'..='9' => {
                let mut end = 0;

                while let Some(&(j, c)) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            chars.next();
                        }
                        _ => {
                            end = j;
                            break;
                        }
                    }
                }

                if let Some(&(_, c)) = chars.peek() {
                    if c == '.' {
                        chars.next();

                        while let Some(&(j, c)) = chars.peek() {
                            match c {
                                '0'..='9' => {
                                    chars.next();
                                }
                                _ => {
                                    end = j;
                                    break;
                                }
                            }
                        }
                    }
                }

                let t = Token {
                    content: &CODE[i..end],
                    line: line,
                };

                tokens.push(t);
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while let Some(&(j, c)) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                            chars.next();
                        }
                        _ => {
                            let t = Token {
                                content: &CODE[i..j],
                                line: line,
                            };

                            tokens.push(t);

                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    let mut line = 1;
    for t in tokens {
        while t.line > line {
            print!("\n{line:<4}: ");
            line += 1;
        }
        print!("{} ", t.content);
    }
}

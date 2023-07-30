mod token;
mod parser;

use crate::token::Token;

fn main() {
    let code: &str = "
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

    let tokens: Vec<Token> = parser::parse(code);
    token::debug_tokens(&tokens);
}

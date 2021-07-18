use crate::tokens::Token::{self, *};

pub fn tokenize(input: &str) -> Vec<Token> {
    // Probably won't have more than 5 bytes at once
    let mut num_buffer = String::with_capacity(5);
    // Most ops are a single byte
    let mut output = Vec::with_capacity(input.len());

    for chr in input.chars() {
        if !num_buffer.is_empty() {
            if Some(&Minus) == output.last() {
                output.pop();
                num_buffer.insert(0, '-');
            }

            if chr.is_numeric() || (chr == '.' && !num_buffer.contains('.')) {
                num_buffer.push(chr);
                continue;
            }

            output.push(Number(
                num_buffer.parse().expect("Failed to parse the float"),
            ));
            num_buffer.clear();
        }

        match chr {
            '0'..='9' => num_buffer.push(chr),
            '=' => output.push(Equal),
            '*' => output.push(Star),
            '/' => output.push(Slash),
            '\\' => output.push(ForwardSlash),
            '&' => output.push(Ampersand),
            '^' => output.push(UpArrow),
            'v' => output.push(DownArrow),
            '<' => output.push(LeftArrow),
            '>' => output.push(RightArrow),
            ':' => output.push(Colon),
            '.' => output.push(Period),
            ',' => output.push(Comma),
            '+' => output.push(Plus),
            '-' => output.push(Minus),
            '@' => output.push(AtSign),
            '%' => output.push(Percentage),
            '~' => output.push(Tilde),
            '|' => output.push(Pipe),
            '_' => output.push(Underscore),
            '#' => output.push(Pound),
            '!' => output.push(Exclamation),
            '[' => output.push(LeftBracket),
            ']' => output.push(RightBracket),
            '$' => output.push(DollarSign),
            '\n' => output.push(Linefeed),
            // Ignore other characters
            _ => {}
        }
    }

    // If final value in stream was a number, this will be true
    if !num_buffer.is_empty() {
        output.push(Number(
            num_buffer.parse().expect("Failed to parse the float"),
        ));
    }

    output.shrink_to_fit();
    output
}

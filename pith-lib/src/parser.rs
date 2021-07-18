use std::array::IntoIter;

use crate::tokens::Token::{self, *};
use crate::utils::Vector as PithVec;

// Convert [ f64 , f64 ] to Vector(f64, f64)
//         RightArrow    to Jmp(usize)
pub fn pre_process(mut stream: Vec<Token>) -> Vec<Token> {
    let mut indexes = Vec::new();
    let mut line: usize = 1;
    let mut i = 0;

    stream.insert(0, Line(0));

    // No iterators so I can mutate the stream while going through it
    while i < stream.len() {
        let tok = stream[i];

        if tok == LeftBracket {
            assert!(
                stream.len() >= i + 5,
                "Invalid vector expression: too short"
            );
            assert_eq!(
                stream[i + 2],
                Comma,
                "Invalid vector expression: missing comma"
            );
            assert_eq!(
                stream[i + 4],
                RightBracket,
                "Invalid vector expression: missing closing bracket"
            );
            stream.splice(
                i..i + 5,
                IntoIter::new([Vector(
                    stream[i + 1].into_value(),
                    stream[i + 3].into_value(),
                )]),
            );
        } else if tok == Linefeed {
            stream[i] = Line(line);
            line += 1;
        }

        i += 1;
    }

    // Matching indexes are handled after other changes
    for (i, tok) in stream.iter_mut().enumerate() {
        if *tok == LeftArrow {
            indexes.push(i);
        } else if *tok == RightArrow {
            *tok = Jmp(indexes
                .pop()
                .expect("'>' requires a matching '<' to occur before it"));
        }
    }

    stream.shrink_to_fit();
    stream
}

pub fn parse(program: &[Token], mut input: Vec<u8>) {
    let mut i: usize = 0;

    let mut targets = (true, true);
    let mut lstack: Vec<f64> = Vec::new();
    let mut rstack: Vec<f64> = Vec::new();
    let mut cstack: Vec<PithVec> = Vec::new();

    while i < program.len() {
        match program[i] {
            Equal => {
                let left = lstack
                    .pop()
                    .expect("Left stack is empty, expected value due to '='");
                let right = rstack
                    .pop()
                    .expect("Right stack is empty, expected value due to '='");
                cstack.push(PithVec(left, right));
            }
            Star => {
                let v = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '*'");
                lstack.push(v.0);
                rstack.push(v.1);
            }
            Slash => rstack.push(
                lstack
                    .pop()
                    .expect("Left stack is empty, expected value due to '/'"),
            ),
            ForwardSlash => lstack.push(
                rstack
                    .pop()
                    .expect("Right stack is empty, expected value due to '/'"),
            ),
            Ampersand => {
                let top = *cstack
                    .last()
                    .expect("Control stack is empty, expected vector due to '&'");
                let value = if let Number(n) = program.get(i + 1).unwrap_or(&Number(1.)) {
                    i += 1;
                    *n as usize
                } else {
                    1
                };

                for _ in 0..value {
                    cstack.push(top);
                }
            }
            // Don't error if no more values, just discard
            UpArrow => {
                lstack.pop();
            }
            DownArrow => {
                rstack.pop();
            }
            Colon => {
                i += 1;
                let value = *program.get(i).expect("Expected value to proceed ':'");
                if let Number(n) = value {
                    cstack.push(PithVec(n, 0.));
                } else if let Vector(l, r) = value {
                    cstack.push(PithVec(l, r));
                } else {
                    panic!(
                        "Expected Number or Vector value to proceed ':', instead got {:?}",
                        value
                    );
                }
            }
            Period => {
                let v1 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '+'");
                let v2 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '+'");
                let dot = v1.dot(v2);
                lstack.push(dot);
                rstack.push(dot);
            }
            Comma => {
                let mag = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to ','")
                    .magnitude();
                i += 1;
                let value = *program.get(i).expect("Expected value to proceed ','");
                if let Number(n) = value {
                    if mag == 0. {
                        i = program
                            .iter()
                            .position(|&tok| tok == Line(n as usize))
                            .unwrap_or(std::usize::MAX);
                    }
                } else {
                    panic!("Value that proceeds ',' must be of type Number");
                }
            }
            Plus => {
                let v1 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '+'");
                let v2 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '+'");
                cstack.push(v1 + v2);
            }
            Minus => {
                let v1 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '-'");
                let v2 = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '-'");
                cstack.push(v2 - v1);
            }
            AtSign => {
                let sign = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '@'")
                    .magnitude()
                    .signum();
                lstack.push(sign);
                rstack.push(sign);
            }
            Percentage => {
                let PithVec(l, r) = cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '%'");
                lstack.push(l.abs());
                rstack.push(r.abs());
            }
            // TODO: Third math operation
            Tilde => todo!(),
            Pipe => print!(
                "{}",
                cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '|'")
                    .magnitude() as u8 as char
            ),
            Underscore => print!(
                "{}",
                cstack
                    .pop()
                    .expect("Control stack is empty, expected vector due to '_'")
                    .magnitude()
            ),
            Pound => {
                let value = *program.get(i + 1).unwrap_or(&Number(1.));
                let value = if let Number(n) = value {
                    i += 1;
                    n as usize
                } else {
                    1
                };

                for _ in 0..value {
                    if !input.is_empty() {
                        input.rotate_left(1);
                    }
                    let v = input.pop().unwrap_or(0x0);
                    cstack.push(PithVec(v as f64, 0.));
                }
            }
            Exclamation => break,
            DollarSign => {
                if targets.0 && targets.1 {
                    targets.1 = false;
                } else if targets.0 {
                    targets = (false, true);
                } else {
                    targets = (true, true);
                }
            }
            Number(_) | Vector(_, _) => panic!("Freestanding {:?}", program[i]),
            Jmp(index) => i = index,
            _ => {}
        }

        i += 1;
    }

    // Trailing newline
    println!()
}

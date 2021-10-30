
use std::io::{self, Read};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    START,
    CHAR(char),
    SPACE(char),
    H_O(u8),
    H_C(u8),
    P_O,
    P_C,
}

pub fn convert(input: String) -> String
{
    build_html_from_tokens(tokenize(input))
}

fn tokenize(input: String) -> Vec<Token>
{
    let mut tokens = Vec::new();

    let mut prev_tag = Token::START;
    let mut prev_token = Token::START;

    for c in input.chars() {
        match prev_token {
            Token::START => {   
                match c {
                    '#' => {
                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);

                        continue;
                    }
                    ' ' | '\n' => { 
                        tokens.push(Token::P_O);

                        prev_token = Token::P_O;
                        prev_tag = Token::P_O;

                        continue;
                    }
                    _ => {
                        tokens.push(Token::P_O);
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        prev_tag = Token::P_O;

                        continue;
                    }
                }
            }
            // ---- Tag openers
            Token::P_O => {
                match c {
                    '#' => {
                        tokens.push(Token::P_C);

                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);

                        continue;
                    }
                    ' ' | '\n' => { continue; }
                    _ => {
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);

                        continue;
                    }
                }
            }
            Token::H_O(i) => {
                match c {
                    '#' => {
                        prev_token = Token::H_O(i+1);
                        prev_tag = Token::H_O(i+1);

                        continue;
                    }
                    ' ' => { 
                        tokens.push(Token::H_O(i));

                        prev_token = Token::SPACE(' ');

                        continue; 
                    }
                    '\n' => {
                        tokens.push(Token::H_C(i));

                        prev_token = Token::H_C(i);

                        continue;
                    }
                    _ => {
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        
                        continue;
                    }
                }
            }
            // ---- Tag closers
            Token::H_C(_) | Token::P_C => {
                match c {
                    ' ' | '\n' => { continue; }
                    '#' => {
                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);

                        continue;
                    }
                    _ => {
                        tokens.push(Token::P_O);
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        prev_tag = Token::P_O;

                        continue;
                    }
                }
            }
            // ---- Characters
            Token::CHAR(_) => {
                match c {
                    '#' => {
                        tokens.push(Token::P_C);
                        
                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);
                        
                        continue;
                    }
                    ' ' => {
                        tokens.push(Token::SPACE(' '));

                        prev_token = Token::SPACE(' ');

                        continue;
                    }
                    '\n' => {
                        match prev_tag {
                            Token::H_O(i) => {
                                tokens.push(Token::H_C(i));

                                prev_token = Token::H_C(i);
                            }
                            _ => { 
                                tokens.push(Token::SPACE(' '));
                                prev_token = Token::SPACE('\n'); 
                            }
                        }

                        continue;
                    }
                    _ => {
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        
                        continue;
                    }
                }
            }
            // ---- Spaces
            Token::SPACE('\n') => {
                match c {
                    '\n' => {
                        if (prev_tag == Token::P_O) & (prev_token != Token::P_C) {
                            tokens.push(Token::P_C);

                            prev_token = Token::P_C;
                        }

                        continue;
                    }
                    '#' => {
                        if (prev_tag == Token::P_O) & (prev_token != Token::P_C) {
                            tokens.push(Token::P_C);
                        }
                        
                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);
                        
                        continue;
                    }
                    ' ' => { 
                        tokens.push(Token::SPACE(' '));

                        prev_token = Token::SPACE(' ');

                        continue;
                    }
                    _ => { 
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        
                        continue;
                    }
                }
            }
            Token::SPACE(_) => {
                match c {
                    '#' => {
                        if (prev_tag == Token::P_O) & (prev_token != Token::P_C) {
                            tokens.push(Token::P_C);
                        }
                        
                        prev_token = Token::H_O(1);
                        prev_tag = Token::H_O(1);
                        
                        continue;
                    }
                    '\n' => {
                        prev_token = Token::SPACE('\n');
                        
                        continue;
                    }
                    ' ' => { continue; }
                    _ => {
                        tokens.push(Token::CHAR(c));

                        prev_token = Token::CHAR(c);
                        
                        continue;
                    }
                }
            }
        }
    }

    match prev_tag {
        Token::P_O => { tokens.push(Token::P_C); }
        Token::H_O(i) => { 
            if prev_token != Token::H_C(i) { tokens.push(Token::H_C(i)); }
        }
        _ => ()
    }

    return tokens;
}

fn build_html_from_tokens(tokens: Vec<Token>) -> String
{
    let mut html = String::new();

    let mut prev_space = false;

    for token in tokens {
        match token {
            Token::H_O(i) => { html.push_str(format!("<h{}>", i).as_str()); }
            Token::H_C(i) => { html.push_str(format!("</h{}>\n", i).as_str()); }

            Token::P_O => { html.push_str("<p>"); }
            Token::P_C => { 
                if prev_space { html.pop(); }     // remove extra space
                html.push_str("</p>\n"); 
            }

            Token::CHAR(c) => { html.push(c); }
            Token::SPACE(_) => { 
                if prev_space { () }

                html.push(' ');
                prev_space = true;
            }

            _ => ()
        }

        match token {
            Token::SPACE(_) => (),
            _ => { prev_space = false; }
        }
    }

    return html;
}

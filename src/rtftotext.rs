use std::collections::HashMap;
use std::io::{Read, Write};

use rtf_grimoire::tokenizer::parse as parse_tokens;
use rtf_grimoire::tokenizer::Token;

use crate::error::{Error, ErrorKind, Result};

use crate::repr;

#[derive(Clone, PartialEq)]
enum Attr {
    Text(String),
    Numeric(i32),
}

#[derive(Clone)]
struct State {
    attrs: HashMap<String, Attr>,
}

impl State {
    fn new() -> Self {
        Self {
            attrs: HashMap::new(),
        }
    }

    fn get_attr(&self, attr: &str) -> Option<&Attr> {
        self.attrs.get(attr)
    }

    fn update_for_control_symbol(&mut self, c: char) {
        match c {
            _ => (),
        }
    }

    fn update_for_control_word(&mut self, name: &str, arg: Option<i32>) {
        match name {
            _ => (),
        }
    }

    fn update_for_token_and_get_printable(&mut self, token: &Token) -> Vec<u8> {
        match token {
            Token::ControlSymbol(c) => self.update_for_control_symbol(*c),
            Token::ControlWord { name, arg } => self.update_for_control_word(name, *arg),
            _ => (),
        }

        if let None = self.get_attr("destination") {
            repr::token_to_byte_repr(token)
        } else {
            Vec::new()
        }
    }
}

pub fn parse<R: Read, W: Write>(mut reader: R, writer: W) -> Result<()> {
    let mut data: Vec<u8> = Vec::with_capacity(4096);
    reader
        .read_to_end(&mut data)
        .map_err(Error::from_input_error)?;

    let token_stream =
        parse_tokens(&data).map_err(|e| Error::new(ErrorKind::Parse, None, Some(Box::new(e))))?;

    write_token_stream(writer, &token_stream)
}

fn write_token_stream<W: Write>(mut writer: W, token_stream: &[Token]) -> Result<()> {
    let mut group_level = 0;
    let mut stack: Vec<State> = Vec::with_capacity(4);

    for token in token_stream.iter().filter(|c| c != &&Token::Newline) {
        write!(writer, "{:?}\n", token).map_err(Error::from_output_error)?;
        if let Token::StartGroup = token {
            group_level += 1;
            stack.push(stack.last().map(|x| x.clone()).unwrap_or(State::new()));
        } else if let Token::EndGroup = token {
            group_level -= 1;
            stack.pop();
        } else if let Some(state) = stack.last_mut() {
            writer
                .write(&state.update_for_token_and_get_printable(token))
                .map_err(Error::from_output_error)?;
        } else {
            panic!("TODO: Error for token outside main group");
        }
    }
    Ok(())
}

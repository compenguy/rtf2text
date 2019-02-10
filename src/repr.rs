use rtf_grimoire::tokenizer::Token;

pub fn token_has_repr(token: &Token) -> bool {
    match token {
        Token::ControlSymbol(c) => _symbol_has_repr(*c),
        Token::ControlWord { name, arg } => _word_has_repr(name, *arg),
        Token::Text(_) => true,
        _ => false,
    }
}

fn _symbol_has_repr(symbol: char) -> bool {
    match symbol {
        '~' => true, // Non-breaking space
        '_' => true, // Non-breaking hyphen
        '\\' => true,
        '{' => true,
        '}' => true,
        '\n' => true,
        '\r' => true,
        _ => false,
    }
}

fn _word_has_repr(word: &str, _arg: Option<i32>) -> bool {
    match word {
        "line" => true,
        "par" => true,
        "sect" => true,
        "page" => true,
        "tab" => true,
        "emdash" => true,
        "endash" => true,
        "bullet" => true,
        "lquote" => true,
        "rquote" => true,
        "ldblquote" => true,
        "rdblquote" => true,
        "emspace" => true,
        "enspace" => true,
        "'" => true,
        _ => false,
    }
}

pub fn token_to_byte_repr(token: &Token) -> Vec<u8> {
    match token {
        Token::ControlSymbol(c) => _symbol_to_byte_repr(*c),
        Token::ControlWord { name, arg } => _word_to_byte_repr(name, *arg),
        Token::Text(text) => text.clone(),
        _ => Vec::new(),
    }
}

fn _symbol_to_byte_repr(symbol: char) -> Vec<u8> {
    match symbol {
        '~' => b"\x20".to_vec(), // Non-breaking space
        '_' => b"\x2D".to_vec(), // Non-breaking hyphen
        '\\' => b"\x5C".to_vec(),
        '{' => b"\x7B".to_vec(),
        '}' => b"\x7D".to_vec(),
        '\n' => b"\x0A".to_vec(),
        '\r' => b"\x0D".to_vec(),
        _ => Vec::new(),
    }
}

fn _word_to_byte_repr(word: &str, arg: Option<i32>) -> Vec<u8> {
    match word {
        "line" => b"\n".to_vec(),
        "par" => b"\n\n".to_vec(),
        "sect" => b"\n\n\n".to_vec(),
        "page" => b"\n\n\n\n".to_vec(),
        "tab" => b"\t".to_vec(),
        "emdash" => b"\xD0".to_vec(),
        "endash" => b"\xD1".to_vec(),
        "bullet" => b"\xA5".to_vec(),
        "lquote" => b"\xD4".to_vec(),
        "rquote" => b"\xD5".to_vec(),
        "ldblquote" => b"\xD2".to_vec(),
        "rdblquote" => b"\xD3".to_vec(),
        "emspace" => b"  ".to_vec(),
        "enspace" => b" ".to_vec(),
        "'" => {
            if let Some(byte) = arg {
                [(byte & 0xFF) as u8].to_vec()
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

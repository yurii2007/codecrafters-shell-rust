enum QuoteMode {
    Single,
    Double,
}

pub fn parse_args(input: &str) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }

    let mut result: Vec<String> = Vec::new();

    let chars = input.as_bytes();
    let mut current_arg = String::new();
    let mut current_idx: usize = 0;
    let mut is_concat_toggled = false;
    let mut quote_mode: Option<QuoteMode> = None;

    while current_idx <= input.len() {
        match chars.get(current_idx) {
            Some(ch) => match ch {
                b'"' => {
                    match quote_mode {
                        Some(QuoteMode::Single) => {
                            current_arg.push('"');
                        }
                        Some(QuoteMode::Double) => {
                            if !is_concat_toggled {
                                result.push(std::mem::take(&mut current_arg));
                            }
                            quote_mode = None;
                            is_concat_toggled = true;
                        }
                        None => {
                            quote_mode = Some(QuoteMode::Double);
                            is_concat_toggled = true
                        }
                    }
                    current_idx += 1;
                }
                b'\'' => {
                    match quote_mode {
                        Some(QuoteMode::Double) => {
                            current_arg.push('\'');
                        }
                        Some(QuoteMode::Single) => {
                            if !is_concat_toggled {
                                result.push(std::mem::take(&mut current_arg));
                            }
                            quote_mode = None;
                            is_concat_toggled = true;
                        }
                        None => {
                            quote_mode = Some(QuoteMode::Single);
                            is_concat_toggled = true;
                        }
                    }
                    current_idx += 1;
                }
                b' ' => {
                    if quote_mode.is_some() {
                        current_arg.push(' ');
                    } else if !current_arg.is_empty() {
                        result.push(std::mem::take(&mut current_arg));
                        is_concat_toggled = !is_concat_toggled;
                    } else {
                        is_concat_toggled = false;
                    }

                    current_idx += 1;
                }
                b'\\' => {
                    if quote_mode.is_none() && chars.get(current_idx + 1).is_some() {
                        current_arg.push_str(&input[current_idx + 1..current_idx + 2]);
                        current_idx += 1;
                    } else {
                        current_arg.push('\\');
                    }

                    current_idx += 1;
                }
                _ => {
                    current_arg.push_str(&input[current_idx..(current_idx + 1)]);
                    current_idx += 1;
                }
            },
            None => {
                if !current_arg.is_empty() {
                    result.push(current_arg);
                }
                return result;
            }
        }
    }

    result
}

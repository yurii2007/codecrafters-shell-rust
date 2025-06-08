pub fn parse_args(input: &str) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }

    let mut result: Vec<String> = Vec::new();
    let mut current_idx = 0;
    let chars = input.as_bytes();

    let mut append_args = |arg: &str, should_concat: bool| {
        if should_concat && !result.is_empty() {
            if let Some(last_arg) = result.last_mut() {
                last_arg.push_str(arg);
            }
        } else {
            result.push(arg.to_string());
        };
    };

    let mut should_concat = false;

    while current_idx <= chars.len() {
        let current_char = chars.get(current_idx);
        match current_char {
            Some(ch) => match ch {
                b'\"' => {
                    let next_quote_idx = find_after(input, "\"", current_idx);

                    append_args(&input[(current_idx + 1)..next_quote_idx], should_concat);
                    should_concat = check_should_concat(chars, next_quote_idx);

                    current_idx = next_quote_idx + 1;
                }
                b'\'' => {
                    let next_quote_idx = find_after(input, "\'", current_idx);

                    append_args(&input[(current_idx + 1)..next_quote_idx], should_concat);
                    should_concat = check_should_concat(chars, next_quote_idx);

                    current_idx = next_quote_idx + 1;
                }
                b' ' => {
                    should_concat = false;
                    current_idx += 1;
                    continue;
                }
                _ => {
                    let next_delimiter_idx = match input[(current_idx + 1)..]
                        .find(|c| c == '\'' || c == '"' || c == ' ')
                    {
                        Some(idx) => current_idx + idx + 1,
                        None => input.len(),
                    };

                    append_args(&input[current_idx..next_delimiter_idx], should_concat);
                    should_concat = check_should_concat(chars, next_delimiter_idx - 1);

                    current_idx = next_delimiter_idx + 1;
                }
            },
            None => return result,
        }
    }

    result
}

fn find_after(input: &str, pattern: &str, start_idx: usize) -> usize {
    let pattern = input[(start_idx + 1)..].find(pattern);

    match pattern {
        Some(pattern_idx) => start_idx + pattern_idx + 1,
        None => input.len(),
    }
}

fn check_should_concat(chars: &[u8], next_char_idx: usize) -> bool {
    let next_char = chars.get(next_char_idx + 1);

    match next_char {
        Some(ch) => *ch != b' ',
        None => false,
    }
}

// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::path::Path;

pub fn get_statement_data<'a>(tokens: Vec<&'a str>, i: usize) -> String {
    let mut string_data = String::new();
    let mut f: usize = i + 1;

    loop {
        if tokens[f].chars().nth(tokens[f].len() - 1).unwrap() == '{' {
            break;
        } else {
            string_data.push_str(tokens[f]);
            string_data.push(' ');
            f = f + 1;
        }
    }

    string_data
}

pub fn get_flag_data<'a>(_variable: &'a str, n: usize) -> (String, String) {
    let mut flag_name = String::new();
    let mut flag_data = String::new();

    let mut is_data: bool = false;

    for ch in _variable.chars().skip(n) {
        if is_data == true {
            if ch == '"' {
                break;
            }

            flag_data.push(ch);
        } else if ch != '"' {
            if ch == '=' {
                is_data = true;
                continue;
            }

            flag_name.push(ch);
        }
    }

    (flag_name, flag_data)
}

pub fn get_include_dir_data<'a>(_variable: &'a str, n: usize) -> String {
    let mut flag_data = String::new();

    for ch in _variable.chars().skip(n) {
        if ch != '"' {
            flag_data.push(ch);
        }
        else {
            break;
        }
    }

    flag_data
}

pub fn extract_file_name(_path: &str) -> &str {
    let data = Path::new(_path).file_name().unwrap().to_str().unwrap();

    &data
}
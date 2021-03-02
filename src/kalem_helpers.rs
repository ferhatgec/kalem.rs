// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

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

    return string_data;
}
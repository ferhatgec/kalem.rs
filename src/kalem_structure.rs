// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate:: {
    kalem_codegen::kalem_codegen,
    Kalem,
};

use std::fs::File;

use std::io::{
    self,
    BufRead
};

use std::path::Path;

use crate::kalem_codegen::{
    KalemCodegenStruct,
    KalemTokens
};

use crate::kalem_codegen::codegen;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_source(data: Kalem) -> KalemCodegenStruct {
    #[allow(dead_code)]
    let mut _tokens: Vec<&str>;

    let mut codegen = KalemCodegenStruct {
        kalem_generated: "".to_string(),
    };

    if let Ok(lines) = read_lines(data.kalem_filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    continue;
                }

                let _tokens: Vec<&str> = ip.trim().split(" ").collect();

                for i in 0.._tokens.len() {
                    match _tokens[i].chars().nth(0).unwrap() as char {
                        '#' => {
                            if _tokens[i] == format!("#{}", codegen::_KALEM_IMPORT).as_str() {
                                kalem_codegen(KalemTokens::KalemImport, &mut codegen, _tokens[i + 1], "");

                            }
                        },
                        '@' => {
                            if _tokens[i] == format!("@{}", codegen::_KALEM_MAIN) {
                                kalem_codegen(KalemTokens::KalemMain, &mut codegen, _tokens[i + 1], "");
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_RETURN) {
                                kalem_codegen(KalemTokens::KalemReturn, &mut codegen, _tokens[i + 1], "");
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_PRINT) {
                                if _tokens[i + 1].chars().next().unwrap() == '"' {
                                    let mut string_data: String = String::new();
                                    let mut f: usize = i + 1;

                                    loop {
                                        string_data.push_str(_tokens[f]);

                                        if _tokens[f].chars().nth(_tokens[f].len()-1).unwrap() == '"' {
                                            break;
                                        }
                                        else {
                                            string_data.push(' ');
                                            f = f + 1;
                                        }
                                    }

                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, string_data.as_str(), "");
                                }
                                else {
                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, _tokens[i + 1], "");
                                }
                            }
                        },
                        '/' => if _tokens[i].chars().nth(1).unwrap() == '/' {},
                        '{' => kalem_codegen(KalemTokens::KalemLeftCurlyBracket, &mut codegen, "", ""),
                        '}' => kalem_codegen(KalemTokens::KalemRightCurlyBracket, &mut codegen, "", ""),
                        _ => {
                            if _tokens[i] == codegen::_KALEM_STRING {
                                if _tokens[i + 2].chars().next().unwrap() == '=' {
                                    if _tokens[i + 3].chars().next().unwrap() == '"' {
                                        let mut string_data: String = String::new();
                                        let mut f: usize = i + 1;

                                        loop {
                                            string_data.push_str(_tokens[f]);

                                            if _tokens[f].chars().nth(_tokens[f].len() - 1).unwrap() == '"' {
                                                break;
                                            }
                                            else {
                                                string_data.push(' ');
                                                f = f + 1;
                                            }
                                        }

                                        kalem_codegen(KalemTokens::KalemString, &mut codegen, string_data.as_str(), "");
                                    }
                                    else {
                                        // Syntax error (string x =)
                                    }
                                }
                            }
                        }
                    }
                }
                kalem_codegen(KalemTokens::KalemNewline, &mut codegen, "", "");
            }
        }
    }

    return codegen;
}
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate:: {
    kalem_codegen::kalem_codegen,
    kalem_helpers::get_statement_data,
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
    KalemTokens,
    codegen
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_source(data: Kalem) -> KalemCodegenStruct {
    let mut _tokens: Vec<&str>;

    let mut is_argument: bool = false;
    let mut is_main: bool = false;

    let mut vec_size;

    let mut codegen = KalemCodegenStruct {
        kalem_generated: "".to_string(),

        kalem_output: "".to_string(),
        kalem_cpp_standard: "c++17".to_string(),
    };

    if let Ok(lines) = read_lines(data.kalem_filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    continue;
                }

                let _tokens: Vec<&str> = ip.trim().split(" ").collect();

                vec_size = _tokens.len();

                for i in 0.._tokens.len() {
                    match _tokens[i].chars().nth(0).unwrap() as char {
                        codegen::SHARP => {
                            if _tokens[i] == format!("#{}", codegen::_KALEM_IMPORT).as_str() {
                                kalem_codegen(KalemTokens::KalemImport, &mut codegen, _tokens[i + 1], "", "");
                            }
                            else if _tokens[i] == format!("#{}", codegen::_KALEM_DEFINE).as_str() {
                                if _tokens[i + 2].chars().next().unwrap() == '"' {
                                    let mut string_data: String = String::new();
                                    let mut f: usize = i + 2;

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

                                    kalem_codegen(KalemTokens::KalemDefine, &mut codegen, string_data.as_str(), _tokens[i + 1], "");
                                }
                                else {
                                    kalem_codegen(KalemTokens::KalemDefine, &mut codegen, _tokens[i + 2], _tokens[i + 1], "");
                                }
                            }
                            else {
                                // To directly use C & C++ code
                                kalem_codegen(KalemTokens::KalemLink, &mut codegen, ip.trim_start(), "", "");
                            }
                        },
                        codegen::FUNCTION_NOTATION => {
                            if _tokens[i] == format!("@{}", codegen::_KALEM_MAIN) {
                                kalem_codegen(KalemTokens::KalemMain, &mut codegen, _tokens[i + 1], "", "");
                                is_argument = true;
                                is_main = true;
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_RETURN) {
                                kalem_codegen(KalemTokens::KalemReturn, &mut codegen, _tokens[i + 1], "", "");
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_PRINT) {
                                if _tokens[i + 1].chars().next().unwrap() == '"' {
                                    let mut string_data = String::new();
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

                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, string_data.as_str(), "", "");
                                }
                                else {
                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, _tokens[i + 1], "", "");
                                }
                            }
                            else {
                                let mut arguments = String::new();
                                let mut function_name= String::new();

                                if i + 2 < vec_size {
                                    if _tokens[i + 2].chars().next().unwrap() == codegen::LEFT_CURLY_BRACKET {
                                        if is_argument == false && is_main == false {
                                            if _tokens[i + 1] == codegen::_KALEM_NAMESPACE {
                                                kalem_codegen(KalemTokens::KalemNamespace, &mut codegen, _tokens[i], _tokens[i + 1], "");
                                            }
                                            else {
                                                kalem_codegen(KalemTokens::KalemFunction, &mut codegen, _tokens[i], _tokens[i + 1], "");
                                            }
                                        }
                                    }
                                    else {
                                        let mut function_type = String::new();

                                        let mut f: usize = 0;

                                        /*
                                            @test(string test, int a) void {

                                            }
                                         */

                                        // TODO: Implement get_element(String, usize, char)
                                        // Get function name
                                        loop {
                                            if f + 1 >= ip.len() {
                                                // Parse error
                                                break;
                                            }

                                            function_name.push(ip.chars().nth(f).unwrap());

                                            if ip.chars().nth(f + 1).unwrap() == '(' {
                                                f = f + 2;
                                                break;
                                            }
                                            else {
                                                f = f + 1;
                                            }
                                        }

                                        // Get arguments
                                        loop {
                                            arguments.push(ip.chars().nth(f).unwrap());

                                            if f + 1 >= ip.len() {
                                                // Parse error
                                                break;
                                            }

                                            if ip.chars().nth(f + 1).unwrap() == ')' {
                                                f = f + 2;

                                                break;
                                            }
                                            else {
                                                f = f + 1;
                                            }
                                        }

                                        if is_main == false {
                                            loop {
                                                if f + 1 >= ip.len() {
                                                    // Parse error
                                                    break;
                                                }

                                                function_type.push(ip.chars().nth(f).unwrap());


                                                if ip.chars().nth(f + 1).unwrap() == ip.chars()
                                                                                        .rev()
                                                                                        .next()
                                                                                        .unwrap() {
                                                    break;
                                                }
                                                else {
                                                    f = f + 1;
                                                }
                                            }
                                        }

                                        function_name = function_name.trim().to_string();
                                        function_type = function_type.trim().to_string();

                                        if is_main == false {
                                            kalem_codegen(KalemTokens::KalemFunction, &mut codegen, function_name.as_str(), function_type.as_str(), arguments.as_str());
                                        }
                                        else if is_main == true && function_type.is_empty() {
                                            if !arguments.is_empty() {
                                                // Function call with arguments
                                                kalem_codegen(KalemTokens::KalemFunctionCall, &mut codegen, function_name.as_str(), "", arguments.as_str());
                                            }
                                            else {
                                                // Function call without arguments
                                                kalem_codegen(KalemTokens::KalemFunctionCall, &mut codegen, function_name.as_str(), "", "");
                                            }
                                        }

                                        // Function call
                                        //kalem_codegen(KalemTokens::KalemFunctionCall, &mut codegen, _tokens[i], "");
                                    }
                                }
                                else {
                                    let mut f: usize = 0;

                                    // Get function name
                                    loop {
                                        function_name.push(ip.chars().nth(f).unwrap());

                                        if f + 1 >= ip.len() {
                                            // Parse error
                                            break;
                                        }

                                        if ip.chars().nth(f + 1).unwrap() == '(' {
                                            f = f + 2;
                                            break;
                                        } else {
                                            f = f + 1;
                                        }
                                    }

                                    // Get arguments
                                    loop {
                                        if f + 1 >= ip.len() {
                                            // Parse error
                                            break;
                                        }

                                        arguments.push(ip.chars().nth(f).unwrap());

                                        if ip.chars().nth(f + 1).unwrap() == ')' {
                                            break;
                                        }
                                        else {
                                            f = f + 1;
                                        }
                                    }

                                    function_name = function_name.trim().to_string();
                                    arguments = arguments.trim_start().to_string();

                                    if !arguments.is_empty() {
                                        // Function call with arguments
                                        kalem_codegen(KalemTokens::KalemFunctionCall, &mut codegen, function_name.as_str(), "", arguments.as_str());
                                    }
                                    else {
                                        kalem_codegen(KalemTokens::KalemFunctionCall, &mut codegen, function_name.as_str(), "", "");
                                    }
                                }
                            }
                        },
                        codegen::FLAG_START => {
                            kalem_codegen(KalemTokens::KalemFlag, &mut codegen, "", ip.as_str(), "");
                        },
                        codegen::SLASH => if _tokens[i].chars().nth(1).unwrap() == '/' {},
                        codegen::LEFT_CURLY_BRACKET => kalem_codegen(KalemTokens::KalemLeftCurlyBracket, &mut codegen, "", "", ""),
                        codegen::RIGHT_CURLY_BRACKET => kalem_codegen(KalemTokens::KalemRightCurlyBracket, &mut codegen, "", "", ""),
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
                                        kalem_codegen(KalemTokens::KalemString, &mut codegen, string_data.as_str(), "", "");
                                    }
                                    else {
                                        // Syntax error (string x =)
                                    }
                                }
                            }
                            else if _tokens[i] == codegen::_KALEM_INT
                                ||  _tokens[i] == codegen::_KALEM_UNSIGNED {
                                if is_argument == true {
                                    is_argument = false;
                                }
                                else if _tokens[i + 2].chars().next().unwrap() == codegen::EQUAL {
                                    let x = if _tokens[i] == codegen::_KALEM_INT {
                                        KalemTokens::KalemInt
                                    }
                                    else {
                                        KalemTokens::KalemUnsigned
                                    };

                                    kalem_codegen(x, &mut codegen, _tokens[i + 3], _tokens[i + 1], "");
                                }
                            }
                            else if _tokens[i] == codegen::_KALEM_IF {
                                kalem_codegen(KalemTokens::KalemIf,
                                              &mut codegen,
                                              get_statement_data(_tokens.to_vec(), i).as_str(),
                                              "",
                                              "");
                            }
                            else if _tokens[i] == codegen::_KALEM_ELSE_IF {
                                kalem_codegen(KalemTokens::KalemElseIf,
                                              &mut codegen,
                                              get_statement_data(_tokens.to_vec(), i).as_str(),
                                              "",
                                              "");
                            }
                            else if _tokens[i] == codegen::_KALEM_ELSE {
                                kalem_codegen(KalemTokens::KalemElse, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_LOOP {
                                kalem_codegen(KalemTokens::KalemLoop, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_CONTINUE {
                                kalem_codegen(KalemTokens::KalemContinue, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_BREAK {
                                kalem_codegen(KalemTokens::KalemBreak, &mut codegen, "", "", "");
                            }
                        }
                    }
                }
                kalem_codegen(KalemTokens::KalemNewline, &mut codegen, "", "", "");
            }
        }
    }

    return codegen;
}
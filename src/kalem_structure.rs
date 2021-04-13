// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate:: {
    kalem_codegen::kalem_codegen,
    kalem_codegen::KalemCase,

    kalem_helpers::get_statement_data,
    kalem_helpers::get_case,

    kalem_types::is_numeric_data,

    kalem_handler::*,

    Kalem
};

use std::io::{
    self,
    BufRead
};

use std::fs::File;
use std::path::Path;

use crate::kalem_codegen::{
    KalemCodegenStruct,
    KalemTokens,
    codegen
};

use crate::kalem_helpers::{get_string_data, get_notation_data};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_source(data: Kalem) -> KalemCodegenStruct {
    let mut _tokens: Vec<&str>;

    let mut is_argument : bool = false;
    let mut is_main     : bool = false;
    let mut is_class    : bool = false;
    let mut is_function : bool = false;
    let mut is_switch   : bool = false;
    let mut is_case     : bool = false;
    let mut is_variable : bool = false;
    let mut is_include  : bool = false;
    let mut is_statement: bool = false;
    let mut is_defn     : bool = false;
    let mut is_flag     : bool = false;
    let mut is_string   : bool = false;

    let mut vec_size;

    let mut var_type: KalemTokens = KalemTokens::KalemUndefined;

    let mut codegen = KalemCodegenStruct {
        kalem_generated   : "".to_string(),

        kalem_output      : "".to_string(),

        kalem_cpp_standard: "c++17".to_string(),
        kalem_cpp_flags   : "-lstdc++fs".to_string(),
        kalem_cpp_dirs    : "-I/usr/include/kalem/stl/ ".to_string(),
        kalem_cpp_compiler: "clang++".to_string(),
        kalem_cpp_sysroot : "".to_string(),
        kalem_structure   : data.kalem_filename.clone(),

        kalem_source_files: vec![],

        kalem_cpp_output  : false,
        kalem_library     : false,

        kalem_ignore_case_warnings: false
    };

    if let Ok(lines) = read_lines(data.kalem_filename.clone()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    continue;
                }

                let _tokens: Vec<&str> = ip.trim().split(" ").collect();
                let mut i: usize = 0;

                vec_size = _tokens.len();

                while i < vec_size {
                    match _tokens[i].chars().nth(0).unwrap() as char {
                        codegen::SLASH => {
                            // if _tokens[i].chars().nth(1).unwrap() == '/' {}

                            break;
                        },
                        codegen::SHARP => {
                            // Import directive does not support multi-line.
                            //
                            // #include {
                            //  <ios>
                            //  <stdstr>
                            //  "my_file"
                            // }
                            //
                            if _tokens[i] == format!("#{}", codegen::_KALEM_IMPORT).as_str() {
                                kalem_codegen(KalemTokens::KalemImport, &mut codegen, _tokens[i + 1], "", "");
                            }
                            else if _tokens[i] == format!("#{}", codegen::_KALEM_INCLUDE).as_str() {
                                if _tokens[i + 1].chars().next().unwrap() == codegen::LEFT_CURLY_BRACKET
                                    && !is_main {
                                    is_include = true;
                                    break;
                                }

                                kalem_codegen(KalemTokens::KalemInclude, &mut codegen, _tokens[i + 1], "", "");
                            }
                            else if _tokens[i] == format!("#{}", codegen::_KALEM_DEFINE).as_str() {
                                if _tokens[i + 1].chars().next().unwrap() == codegen::LEFT_CURLY_BRACKET {
                                    is_defn  = true;
                                    var_type = KalemTokens::KalemDefine;

                                    break;
                                }
                                else if _tokens[i + 2].chars().next().unwrap() == codegen::QUOTATION_MARK {
                                    kalem_codegen(KalemTokens::KalemDefine, &mut codegen, get_string_data(_tokens.clone(), i, false).as_str(), _tokens[i + 1], "");
                                }
                                else {
                                    kalem_codegen(KalemTokens::KalemDefine, &mut codegen, _tokens[i + 2], _tokens[i + 1], "");
                                }
                            }
                            else {
                                // To directly use C & C++ code
                                kalem_codegen(KalemTokens::KalemLink, &mut codegen, ip.trim_start(), "", "");
                                break;
                            }
                        },
                        codegen::FUNCTION_NOTATION => {
                            if _tokens[i] == format!("@{}", codegen::_KALEM_MAIN) {
                                kalem_codegen(KalemTokens::KalemMain, &mut codegen, _tokens[i + 1], "", "");
                                is_argument = true;
                                is_main = true;
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_RETURN) {
                                kalem_codegen(KalemTokens::KalemReturn, &mut codegen, get_notation_data(_tokens.clone(), i).as_str(), "", "");
                            }
                            else if _tokens[i] == format!("@{}", codegen::_KALEM_PRINT) {
                                if _tokens[i + 1].chars().next().unwrap() == '"' {
                                    let start = ip.find("\"").unwrap_or(0);

                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, &ip[start..ip.len()], "", "");
                                }
                                else {
                                    kalem_codegen(KalemTokens::KalemPrint, &mut codegen, get_notation_data(_tokens, i).as_str(), "", "");
                                }

                                break;
                            }
                            else if _tokens[i].contains("=") {
                                kalem_codegen(KalemTokens::KalemUndefined, &mut codegen, _tokens[i].chars().next().map(|c| &_tokens[i][c.len_utf8()..]).unwrap(), "", "");
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
                                            else if _tokens[i + 1] == codegen::_KALEM_CLASS {
                                                kalem_codegen(KalemTokens::KalemClass, &mut codegen, _tokens[i], "", "");

                                                is_class = true;
                                            }
                                            else {
                                                // TODO: Create case_warning() function
                                                if !codegen.kalem_ignore_case_warnings {
                                                    match get_case(function_name.as_str()) {
                                                        KalemCase::PascalCase => {
                                                            KalemErrorData::output(KalemErrors::HelpCase,
                                                                                   data.kalem_filename.to_owned(),
                                                                                   ip.as_str(),
                                                                                   function_name.as_str(),
                                                                                   "snake_case");
                                                        },
                                                        _ => {}
                                                    }
                                                }

                                                kalem_codegen(KalemTokens::KalemFunction, &mut codegen, _tokens[i], _tokens[i + 1], "");
                                                is_function = true;
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
                                            // In Kalem, function names should be written
                                            // as snake_case on by default.
                                            if !codegen.kalem_ignore_case_warnings {
                                                match get_case(function_name.as_str()) {
                                                    KalemCase::PascalCase => {
                                                        KalemErrorData::output(KalemErrors::HelpCase,
                                                                               data.kalem_filename.to_owned(),
                                                                               ip.as_str(),
                                                                               function_name.as_str(),
                                                                               "snake_case");
                                                    },
                                                    _ => {}
                                                }
                                            }

                                            kalem_codegen(KalemTokens::KalemFunction, &mut codegen, function_name.as_str(), function_type.as_str(), arguments.as_str());
                                            i = i + 2;
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

                                        if ip.chars().nth(f + 1).unwrap() == ')' && f + 1 == ip.len() {
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
                            if ip.contains(format!("!{}", codegen::_KALEM_FLAG).as_str()) {
                                if i + 1 < vec_size {
                                    if _tokens[i + 1].chars().next().unwrap() == codegen::LEFT_CURLY_BRACKET {
                                        is_flag = true;

                                        break;
                                    }
                                }

                                kalem_codegen(KalemTokens::KalemFlag, &mut codegen, "", ip.as_str(), "");
                            }
                            else if ip.contains(format!("!{}", codegen::_KALEM_REQUIRED_FLAG).as_str()) {
                                kalem_codegen(KalemTokens::KalemRequiredFlag, &mut codegen, data.kalem_filename.clone().as_str(), ip.as_str(), "");
                            }
                            else if ip.contains(format!("!{}", codegen::_KALEM_ADD_SOURCE).as_str()) {
                                kalem_codegen(KalemTokens::KalemAddSource, &mut codegen, "", ip.as_str(), "");
                            }
                            else if ip.contains(format!("!{}", codegen::_KALEM_INCLUDE_DIR).as_str()) {
                                kalem_codegen(KalemTokens::KalemIncludeDir, &mut codegen, "", ip.as_str(), "");
                            }
                        },
                        codegen::LEFT_CURLY_BRACKET =>  kalem_codegen(KalemTokens::KalemLeftCurlyBracket, &mut codegen, "", "", ""),
                        codegen::RIGHT_CURLY_BRACKET => {
                            if is_statement {
                                is_statement= false;
                            }
                            else if is_string {
                                is_string   = false;

                                break;
                            }
                            else if is_flag {
                                is_flag     = false;

                                break;
                            }
                            else if is_defn {
                                is_defn     = false;

                                break;
                            }
                            else if is_variable {
                                is_variable = false;

                                break;
                            }
                            else if is_include {
                                is_include  = false;

                                break;
                            }
                            else if is_class && !is_function {
                                kalem_codegen(KalemTokens::KalemRightCurlyBracket, &mut codegen, ";", "", "");
                                is_class    = false;

                                break;
                            }
                            else if is_function {
                                is_function = false;
                            }
                            else if is_case && is_switch {
                                is_case     = false;
                            }
                            else if is_case && is_switch {
                                is_switch   = false;
                            }

                            kalem_codegen(KalemTokens::KalemRightCurlyBracket, &mut codegen, "", "", "");

                            break;
                        },
                        codegen::MEMBER => {
                            if is_class {
                                kalem_codegen(KalemTokens::KalemClassMemberVisibility, &mut codegen, _tokens[i], "", "");
                            }
                            else if is_switch {
                                kalem_codegen(KalemTokens::KalemCase, &mut codegen, _tokens[i], "", "");
                            }
                        },
                        _ => {
                            if is_string {
                                if _tokens[i + 1].chars().next().unwrap() == codegen::QUOTATION_MARK {
                                    kalem_codegen(var_type, &mut codegen, get_string_data(_tokens.clone(), i, true).as_str(), _tokens[i], "");
                                } else {
                                    kalem_codegen(var_type, &mut codegen, "", _tokens[i], "");
                                };

                                break;
                            }

                            if is_flag {
                                kalem_codegen(KalemTokens::KalemFlag, &mut codegen, "", ip.trim(), "");

                                break;
                            }

                            if is_variable || is_defn {
                                let x = var_type;

                                kalem_codegen(x, &mut codegen, _tokens[i + 1], _tokens[i], "");

                                break;
                            }

                            if is_include {
                                match _tokens[i].chars().next().unwrap() {
                                    codegen::LESS_THAN | codegen::QUOTATION_MARK => {
                                        kalem_codegen(KalemTokens::KalemInclude, &mut codegen, _tokens[i], "", "");
                                    },
                                    _=> { continue; }
                                }

                                break;
                            }

                            if _tokens[i] == codegen::_KALEM_STRING
                                || _tokens[i] == codegen::_KALEM_STR {
                                if _tokens[i + 1].chars().next().unwrap() == codegen::LEFT_CURLY_BRACKET {
                                    is_string = true;

                                    var_type = if _tokens[i] == codegen::_KALEM_STRING {
                                        KalemTokens::KalemString
                                    } else {
                                        KalemTokens::KalemStr
                                    };

                                    break;
                                }
                                else if _tokens[i + 2].chars().next().unwrap() != codegen::EQUAL {
                                    let x = if _tokens[i] == codegen::_KALEM_STRING {
                                        KalemTokens::KalemString
                                    } else {
                                        KalemTokens::KalemStr
                                    };

                                    if _tokens[i + 2].chars().next().unwrap() == codegen::QUOTATION_MARK {
                                        kalem_codegen(x, &mut codegen, get_string_data(_tokens.clone(), i, false).as_str(), _tokens[i + 1], "");
                                    } else {
                                        kalem_codegen(x, &mut codegen, "", _tokens[i + 1], "");
                                    };
                                }
                            }
                            else if _tokens[i] == codegen::_KALEM_INT
                                ||  _tokens[i] == codegen::_KALEM_UNSIGNED
                                ||  _tokens[i] == codegen::_KALEM_CHAR {
                                if is_argument == true {
                                    is_argument = false;
                                }
                                else if _tokens[i + 1].chars().nth(0).unwrap() == codegen::LEFT_CURLY_BRACKET {
                                    is_variable = true;
                                    var_type = is_numeric_data(&_tokens, i);

                                    break;
                                }
                                else if _tokens[i + 2].chars().next().unwrap().is_numeric() {
                                    let x = is_numeric_data(&_tokens, i);

                                    kalem_codegen(x, &mut codegen, _tokens[i + 2], _tokens[i + 1], "");
                                }
                                else {
                                    if _tokens[i] == codegen::_KALEM_CHAR {
                                        kalem_codegen(KalemTokens::KalemChar, &mut codegen, _tokens[i + 2], _tokens[i + 1], "");
                                    }
                                }
                            }
                            else if _tokens[i] == codegen::_KALEM_IF {
                                kalem_codegen(KalemTokens::KalemIf,
                                              &mut codegen,
                                              get_statement_data(_tokens.to_vec(), i).as_str(),
                                              "",
                                              "");

                                is_statement = true;
                            }
                            else if _tokens[i] == codegen::_KALEM_ELSE_IF {
                                kalem_codegen(KalemTokens::KalemElseIf,
                                              &mut codegen,
                                              get_statement_data(_tokens.to_vec(), i).as_str(),
                                              "",
                                              "");

                                is_statement = true;
                            }
                            else if _tokens[i] == codegen::_KALEM_ELSE {
                                kalem_codegen(KalemTokens::KalemElse, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_LOOP {
                                kalem_codegen(KalemTokens::KalemLoop, &mut codegen, _tokens[i + 1], get_statement_data(_tokens.to_vec(), i).as_str(), "");
                            }
                            else if _tokens[i] == codegen::_KALEM_CONTINUE {
                                kalem_codegen(KalemTokens::KalemContinue, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_BREAK {
                                kalem_codegen(KalemTokens::KalemBreak, &mut codegen, "", "", "");
                            }
                            else if _tokens[i] == codegen::_KALEM_SWITCH {
                                kalem_codegen(KalemTokens::KalemSwitch, &mut codegen, "", _tokens[i + 1], "");
                                is_switch = true;
                            }
                            else if _tokens[i].len() > 1 && ((_tokens[i].chars().last().unwrap() == '+'
                                    && _tokens[i].chars().nth_back(1).unwrap() == '+')
                                || (_tokens[i].chars().last().unwrap() == '-'
                                    && _tokens[i].chars().nth_back(1).unwrap() == '-')
                                || (_tokens[i].chars().next().unwrap() == '+'
                                    && _tokens[i].chars().nth(1).unwrap() == '+')
                                || (_tokens[i].chars().next().unwrap() == '-'
                                    && _tokens[i].chars().nth(1).unwrap() == '-')) {
                                kalem_codegen(KalemTokens::KalemUndefined, &mut codegen, "", _tokens[i], "");
                            }
                        }
                    }

                    i = i + 1;
                }
                kalem_codegen(KalemTokens::KalemNewline, &mut codegen, "", "", "");
            }
        }
    }

    return codegen;
}
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::path::Path;

use crate::kalem_helpers::{
    get_flag_data,
    get_include_dir_data,
    extract_file_name
};

#[allow(dead_code)]
pub mod codegen {
    pub const _KALEM_INT:               &str = "int";
    pub const _KALEM_UNSIGNED:          &str = "unsign";
    pub const _KALEM_STRING:            &str = "string";
    pub const _KALEM_STR:               &str = "str";
    pub const _KALEM_CHAR:              &str = "char";

    pub const _KALEM_IMPORT:            &str = "import";
    pub const _KALEM_INCLUDE:           &str = "include";

    pub const _KALEM_MAIN:              &str = "main";
    pub const _KALEM_RETURN:            &str = "return";
    pub const _KALEM_PRINT:             &str = "print";

    pub const _KALEM_DEFINE:            &str = "defn";

    pub const _KALEM_NAMESPACE:         &str = "namespace";

    pub const _KALEM_CLASS:             &str = "class";

    pub const _KALEM_IF:                &str = "if";
    pub const _KALEM_ELSE:              &str = "els";
    pub const _KALEM_ELSE_IF:           &str = "elsif";

    // pub const _KALEM_WHILE:             &str = "while";

    pub const _KALEM_FLAG:              &str = "flag";
    pub const _KALEM_REQUIRED_FLAG:     &str = "required_flag";
    pub const _KALEM_INCLUDE_DIR:       &str = "include_dir";
    pub const _KALEM_ADD_SOURCE:        &str = "add_source";

    pub const _KALEM_LOOP:              &str = "loop";

    pub const _KALEM_CONTINUE:          &str = "continue";
    pub const _KALEM_BREAK:             &str = "break";

    pub const _KALEM_SWITCH:            &str = "switch";

    pub const _KALEM_VOID:              &str = "void";

    pub const _KALEM_VECTOR:            &str = "vect";

    pub const _KALEM_UNIMPLEMENTED:     char = '?';

    pub const _CPP_KALEM_INT:           &str = "int";
    pub const _CPP_KALEM_UNSIGNED:      &str = "unsigned";
    pub const _CPP_KALEM_STRING:        &str = "string";
    pub const _CPP_KALEM_STR:           &str = "char";
    pub const _CPP_KALEM_CHAR:          &str = "char";

    pub const _CPP_KALEM_IMPORT:        &str = "include";

    pub const _CPP_KALEM_MAIN:          &str = "main";
    pub const _CPP_KALEM_RETURN:        &str = "return";
    pub const _CPP_KALEM_PRINT:         &str = "cout";

    pub const _CPP_KALEM_DEFINE:        &str = "define";

    pub const _CPP_KALEM_NAMESPACE:     &str = "namespace";

    pub const _CPP_KALEM_CLASS:         &str = "class";

    pub const _CPP_KALEM_IF:            &str = "if";
    pub const _CPP_KALEM_ELSE:          &str = "else";

    // pub const _CPP_KALEM_WHILE:         &str = "while";

    pub const _CPP_KALEM_LOOP:          &str = "while";

    pub const _CPP_KALEM_CONTINUE:      &str = "continue";
    pub const _CPP_KALEM_BREAK:         &str = "break";

    pub const _CPP_KALEM_SWITCH:        &str = "switch";
    pub const _CPP_KALEM_CASE:          &str = "case";

    pub const _CPP_KALEM_VOID:          &str = "void";

    pub const _CPP_KALEM_VECTOR:        &str = "vector";

    pub const LEFT_CURLY_BRACKET:       char = '{';
    pub const RIGHT_CURLY_BRACKET:      char = '}';

    pub const FUNCTION_NOTATION:        char = '@';

    pub const SLASH:                    char = '/';

    pub const POINTER:                  char = '*';

    pub const SHARP:                    char = '#';

    pub const SEMICOLON:                char = ';';
    pub const EQUAL:                    char = '=';

    pub const FLAG_START:               char = '!';

    pub const MEMBER:                   char = '~';

    pub const WHITESPACE:               char = ' ';

    pub const NEWLINE:                  char = '\n';
}

pub mod append_codegen {
    pub const _CPP_KALEM_ARGC:          &str = "argc";
    pub const _CPP_KALEM_ARGV:          &str = "argv";

    pub const _CPP_IFNDEF:              &str = "ifndef";
    pub const _CPP_ENDIF:               &str = "endif";
}

#[derive(Clone, Copy)]
pub enum KalemTokens {
    KalemInt = 0,
    KalemUnsigned,
    KalemString,
    KalemStr,
    KalemChar,

    KalemImport,
    KalemInclude,

    KalemMain,
    KalemReturn,
    KalemPrint,

    KalemFunction,
    KalemFunctionCall,

    KalemDefine,

    KalemNamespace,

    KalemClass,
    KalemClassMemberVisibility,


    KalemIf,
    // KalemWhile,
    KalemElse,
    KalemElseIf,

    KalemFlag,
    KalemRequiredFlag,
    KalemIncludeDir,
    KalemAddSource,

    KalemLoop,

    KalemContinue,
    KalemBreak,

    KalemSwitch,
    KalemCase,

    // KalemUnImplemented,

    KalemLink,

    KalemLeftCurlyBracket,
    KalemRightCurlyBracket,

    KalemNewline,

    KalemUndefined,
}

pub struct KalemCodegenStruct {
    pub kalem_generated:    String,

    pub kalem_output:       String,

    pub kalem_cpp_standard: String,
    pub kalem_cpp_flags:    String,
    pub kalem_cpp_dirs:     String,
    pub kalem_cpp_compiler: String,
    pub kalem_cpp_sysroot:  String,

    pub kalem_source_files: Vec<String>,

    pub kalem_cpp_output: bool,
    pub kalem_library   : bool,

    pub kalem_ignore_case_warnings : bool,
}

pub enum KalemCase {
    SnakeCase,
    PascalCase,

    UndefinedCase,
}

pub fn kalem_codegen(token: KalemTokens,
    data: &mut KalemCodegenStruct,
    keyword: &str,
    variable: &str,
    arguments: &str) {
    match token {
        KalemTokens::KalemImport => {
            let mut _keyword = String::from(keyword);

            _keyword = _keyword.replace(".kalem", ".hpp");

            data.kalem_generated.push_str(format!("#{} {}",
                                                  codegen::_CPP_KALEM_IMPORT,
                                                  _keyword).as_str());

            drop(_keyword);
        },
        KalemTokens::KalemInclude => {
            let mut _keyword = String::from(keyword);

            if keyword.contains(".kalem") {
                _keyword = _keyword.replace(".kalem", ".hpp");
            }
            else {
                if _keyword.chars().last().unwrap() == '>' {
                    let mut source_data = String::from(extract_file_name(_keyword.as_str()));

                    // TODO: Add remove_fl helper function
                    let mut characters = source_data.chars();

                    characters.next();
                    characters.next_back();

                    source_data = format!("/usr/include/kalem/stl/{}", characters.as_str().to_string());

                    if Path::new(format!("{}.kalem", source_data).as_str()).exists() {
                        if !data.kalem_source_files.contains(&source_data) {
                            data.kalem_source_files.push(source_data);
                        }
                    }
                    // else {
                        // Throw error
                    //}
                }

                _keyword.pop();

                _keyword.push_str(".hpp");
                _keyword.push(keyword.chars().last().unwrap());
            }

            data.kalem_generated.push_str(format!("#{} {}",
                                                  codegen::_CPP_KALEM_IMPORT,
                                                  _keyword).as_str());

            drop(_keyword);
        },
        KalemTokens::KalemString => {
            data.kalem_generated.push_str(format!("std::{} {}",
                                                  codegen::_CPP_KALEM_STRING,
                                                  variable).as_str());

            if !keyword.is_empty() {
                data.kalem_generated.push_str(format!("{} {}",
                                                      codegen::EQUAL,
                                                      keyword).as_str());
            }

            data.kalem_generated.push(codegen::SEMICOLON);
        },
        KalemTokens::KalemStr => {
            data.kalem_generated.push_str(format!("{} {}[]",
                                                  codegen::_CPP_KALEM_STR,
                                                  variable).as_str());

            if !keyword.is_empty() {
                data.kalem_generated.push_str(format!("{} {}",
                                                      codegen::EQUAL,
                                                      keyword).as_str());
            }

            data.kalem_generated.push(codegen::SEMICOLON);
        },
        KalemTokens::KalemChar => {
            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_CHAR,
                                                  variable).as_str());

            if !keyword.is_empty() {
                data.kalem_generated.push_str(format!("{} {}",
                                                      codegen::EQUAL,
                                                      keyword).as_str());
            }

            data.kalem_generated.push(codegen::SEMICOLON);
        },
        KalemTokens::KalemInt => {
            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_INT,
                                                  variable).as_str());

            if !keyword.is_empty() {
                data.kalem_generated.push(codegen::EQUAL);
                data.kalem_generated.push_str(keyword);
            }

            data.kalem_generated.push(codegen::SEMICOLON);

        },
        KalemTokens::KalemUnsigned => {
            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_UNSIGNED,
                                                  variable).as_str());

            if !keyword.is_empty() {
                data.kalem_generated.push(codegen::EQUAL);
                data.kalem_generated.push_str(keyword);
            }

            data.kalem_generated.push(codegen::SEMICOLON);
        },
        KalemTokens::KalemFunction => {
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();
            let mut argument      = String::from(arguments);

            if !arguments.is_empty() {
                argument = argument.replace(codegen::_KALEM_STRING,
                    format!("std::{}", codegen::_CPP_KALEM_STRING).as_str());

                argument = argument.replace(codegen::_KALEM_UNSIGNED,
                    codegen::_CPP_KALEM_UNSIGNED);

                argument = argument.replace(codegen::_KALEM_VECTOR,
                    codegen::_CPP_KALEM_VECTOR);

                argument = format!("({})", argument);
            }
            else {
                argument = "()".to_string();
            }

            let mut variable = String::from(variable);

            // Unimplemented variable from C or C++
            if variable.chars().last().unwrap() != codegen::_KALEM_UNIMPLEMENTED {
                // TODO: !syntax_rules("function_argument" eq "string" replace "std::string")
                if variable == "string" {
                    variable = variable.replace(&variable, "std::string");
                }
            }
            else {
                if variable.len() > 1 {
                    variable.pop();
                }
            }

            // Function arguments are not supported yet.
            data.kalem_generated.push_str(format!("{} {}{}",
                                                  variable,
                                                  keyword,
                                                  argument).as_str());

            drop(argument);
        },
        KalemTokens::KalemFunctionCall => {
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();
            let mut argument = String::from(arguments);

            if arguments.is_empty() {
                argument.clear();
            }

            data.kalem_generated.push_str(format!("{}({});", keyword, argument).as_str());

            drop(argument);
        },
        KalemTokens::KalemMain => {
            data.kalem_generated.push_str(format!("{} {}({} {}, {}** {})",
                                                  keyword,                          // Type
                                                  codegen::_CPP_KALEM_MAIN,        // Main
                                                  codegen::_CPP_KALEM_INT,         // Argument counter type
                                                  append_codegen::_CPP_KALEM_ARGC, // Argument counter name
                                                  codegen::_CPP_KALEM_CHAR,        // Argv type
                                                  append_codegen::_CPP_KALEM_ARGV  // Argv name
                                                    ).as_str());
        },
        KalemTokens::KalemNamespace => {
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();

            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_NAMESPACE,
                                                  keyword).as_str());
        },
        KalemTokens::KalemClass => {
            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_CLASS,
                                                  keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap()).as_str());
        },
        KalemTokens::KalemClassMemberVisibility => {
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();

            if keyword == "public"
                || keyword == "protected"
                || keyword == "private" {
                data.kalem_generated.push_str(format!("{}:\n", keyword).as_str());
            }
        },
        KalemTokens::KalemIf => {
            data.kalem_generated.push_str(codegen::_CPP_KALEM_IF);
            data.kalem_generated.push_str(format!("({})", keyword).as_str());
        },
        KalemTokens::KalemElse => {
            data.kalem_generated.push_str(codegen::_CPP_KALEM_ELSE);
            data.kalem_generated.push(codegen::WHITESPACE);
        },
        KalemTokens::KalemElseIf => {
            data.kalem_generated.push_str(format!("{} {}({})",
                                                  codegen::_CPP_KALEM_ELSE,
                                                  codegen::_CPP_KALEM_IF,
                                                  keyword).as_str());
        },
        KalemTokens::KalemFlag => {
            if variable.len() > 6 {
                let (flag_name, flag_data) = get_flag_data(variable, 6);

                if flag_name == "output" {
                    data.kalem_output = flag_data;
                }
                else if flag_name == "cpp-standard" {
                    data.kalem_cpp_standard = flag_data;
                }
                else if flag_name == "cpp-output" {
                    if flag_data == "true" {
                        data.kalem_cpp_output = true;
                    }
                    else {
                        data.kalem_cpp_output = false;
                    }
                }
                else if flag_name == "cpp-flags" {
                    if flag_data != "false" {
                        data.kalem_cpp_flags = flag_data;
                    }
                }
                else if flag_name == "cpp-compiler" {
                    if flag_data != "default" && flag_data != data.kalem_cpp_compiler {
                        data.kalem_cpp_compiler = flag_data;
                    }
                }
                else if flag_name == "cpp-sysroot" {
                    if flag_data != "false" {
                        data.kalem_cpp_sysroot = format!("--{}={}", "sysroot", flag_data).to_string();
                    }
                }
                else if flag_name == "ignore-case-warnings" {
                    if flag_data == "true" {
                        data.kalem_ignore_case_warnings = true;
                    }
                    else {
                        data.kalem_ignore_case_warnings = false;
                    }
                }
                else if flag_name == "hash-cache" {
                    println!("info: {}", "Kalem.rs is not supporting hash caching yet.");
                }

                drop(flag_name);
            }
        },
        KalemTokens::KalemRequiredFlag => {
            let flag_data = get_include_dir_data(variable, 16);

            if flag_data == "library" {
                data.kalem_library = true;

                let mut filename = String::from(keyword);

                filename = filename.replace(".kalem", "");

                filename = extract_file_name(&filename).to_string();

                data.kalem_generated = format!("#{} {}_HPP\n#{} {}_HPP\n{}",
                                               append_codegen::_CPP_IFNDEF,
                                               filename,
                                               codegen::_CPP_KALEM_DEFINE,
                                               filename,
                                               &data.kalem_generated);
            }
            else if flag_data == "source" {
                data.kalem_library = false;
            }
        },
        KalemTokens::KalemIncludeDir => {
            // 14 = 1 + _KALEM_INCLUDE_DIR + 2
            let flag_data = get_include_dir_data(variable, 14);

            // TODO: Throw error if directory is not exist.
            if flag_data != "false" {
                data.kalem_cpp_dirs.push_str(format!("-I{} ", flag_data).as_str());
            }
            else {
                data.kalem_cpp_dirs = "".to_string();
            }
        },
        KalemTokens::KalemAddSource => {
            let source_data = get_include_dir_data(variable, 13);

            data.kalem_source_files.push(source_data);
        },
        KalemTokens::KalemLoop => {
            let mut variable = String::from(variable);

            if keyword.is_empty() || keyword == "{" {
                variable = "1".to_string();
            }

            data.kalem_generated.push_str(format!("{}({})",
                                                  codegen::_CPP_KALEM_LOOP,
                                                  variable).as_str());
        },
        KalemTokens::KalemContinue => {
            data.kalem_generated.push_str(format!("{};", codegen::_CPP_KALEM_CONTINUE).as_str());
        },
        KalemTokens::KalemBreak => {
            data.kalem_generated.push_str(format!("{};", codegen::_CPP_KALEM_BREAK).as_str());
        },
        KalemTokens::KalemSwitch => {
            data.kalem_generated.push_str(format!("{}({})",
                                                  codegen::_CPP_KALEM_SWITCH,
                                                  variable).as_str());
        },
        KalemTokens::KalemCase => {
            let mut keyword = String::from(keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap());

            if keyword != "default" {
                keyword = format!("{} {}:", codegen::_CPP_KALEM_CASE, keyword);
            }
            else {
                keyword = format!("{}:", keyword);
            }

            data.kalem_generated.push_str(keyword.as_str());
        },
        KalemTokens::KalemLink => {
            // TODO: Create pop_front() function
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();

            data.kalem_generated.push_str(keyword);
        },
        KalemTokens::KalemDefine => {
            data.kalem_generated.push_str(format!("#{} {} {}",
                                                  codegen::_CPP_KALEM_DEFINE,
                                                  variable,
                                                  keyword).as_str());
        },
        KalemTokens::KalemReturn => {
            data.kalem_generated.push_str(format!("{} {};",
                                                  codegen::_CPP_KALEM_RETURN,
                                                  keyword).as_str());
        },
        KalemTokens::KalemPrint => {
            data.kalem_generated.push_str(format!("std::{}<<{};",
                                                  codegen::_CPP_KALEM_PRINT,
                                                  keyword).as_str());
        },
        KalemTokens::KalemLeftCurlyBracket => {
            data.kalem_generated.push(codegen::LEFT_CURLY_BRACKET);
        },
        KalemTokens::KalemRightCurlyBracket => {
            data.kalem_generated.push(codegen::RIGHT_CURLY_BRACKET);

            if !keyword.is_empty() {
                data.kalem_generated.push(codegen::SEMICOLON);
            }
        },
        KalemTokens::KalemNewline => {
            data.kalem_generated.push(codegen::NEWLINE);
        },
        KalemTokens::KalemUndefined => {
            if !variable.is_empty() {
                data.kalem_generated.push_str(format!("{};", variable).as_str());
            }
            else if !keyword.is_empty() {
                data.kalem_generated.push_str(format!("{};", keyword).as_str());
            }
        },
    }
}
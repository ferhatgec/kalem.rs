// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

#[allow(dead_code)]
pub mod codegen {
    pub const _KALEM_INT:               &str = "int";
    pub const _KALEM_UNSIGNED:          &str = "unsign";
    pub const _KALEM_STRING:            &str = "string";
    pub const _KALEM_CHAR:              &str = "char";

    pub const _KALEM_IMPORT:            &str = "import";
    pub const _KALEM_MAIN:              &str = "main";
    pub const _KALEM_RETURN:            &str = "return";
    pub const _KALEM_PRINT:             &str = "print";

    pub const _KALEM_DEFINE:            &str = "defn";

    pub const _KALEM_NAMESPACE:         &str = "namespace";

    pub const _KALEM_IF:                &str = "if";
    pub const _KALEM_ELSE:              &str = "els";
    pub const _KALEM_ELSE_IF:           &str = "elsif";

    // pub const _KALEM_WHILE:             &str = "while";

    pub const _KALEM_FLAG:              &str = "flag";

    pub const _KALEM_LOOP:              &str = "loop";

    pub const _KALEM_VOID:              &str = "void";

    pub const _KALEM_VECTOR:            &str = "vect";

    pub const _CPP_KALEM_INT:           &str = "int";
    pub const _CPP_KALEM_UNSIGNED:      &str = "unsigned";
    pub const _CPP_KALEM_STRING:        &str = "string";
    pub const _CPP_KALEM_CHAR:          &str = "char";

    pub const _CPP_KALEM_IMPORT:        &str = "include";
    pub const _CPP_KALEM_MAIN:          &str = "main";
    pub const _CPP_KALEM_RETURN:        &str = "return";
    pub const _CPP_KALEM_PRINT:         &str = "cout";

    pub const _CPP_KALEM_DEFINE:        &str = "define";

    pub const _CPP_KALEM_NAMESPACE:     &str = "namespace";

    pub const _CPP_KALEM_IF:            &str = "if";
    pub const _CPP_KALEM_ELSE:          &str = "else";

    // pub const _CPP_KALEM_WHILE:         &str = "while";

    pub const _CPP_KALEM_LOOP:          &str = "while";

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

    pub const WHITESPACE:               char = ' ';

    pub const NEWLINE:                  char = '\n';
}

pub mod append_codegen {
    pub const _CPP_KALEM_ARGC:          &str = "argc";
    pub const _CPP_KALEM_ARGV:          &str = "argv";
}

pub enum KalemTokens {
    KalemInt = 0,
    KalemUnsigned,
    KalemString,

    KalemImport,
    KalemMain,
    KalemReturn,
    KalemPrint,

    KalemFunction,
    KalemFunctionCall,

    KalemDefine,
    KalemNamespace,

    KalemIf,
    // KalemWhile,
    KalemElse,
    KalemElseIf,

    KalemFlag,

    KalemLoop,

    KalemLink,

    KalemLeftCurlyBracket,
    KalemRightCurlyBracket,

    KalemNewline,
}

pub struct KalemCodegenStruct {
    pub kalem_generated: String,

    pub kalem_output: String,
    pub kalem_cpp_standard: String,
}

pub fn kalem_codegen(token: KalemTokens,
    data: &mut KalemCodegenStruct,
    keyword: &str,
    variable: &str,
    arguments: &str) {
    match token {
        KalemTokens::KalemImport => {
            let mut _keyword = String::from(keyword);

            _keyword = _keyword.replace("ios", "iostream");
            _keyword = _keyword.replace("stdstr", "string");
            _keyword = _keyword.replace("vect", "vector");
            _keyword = _keyword.replace("cstdstr", "cstring");
            _keyword = _keyword.replace("iom", "iomanip");
            _keyword = _keyword.replace("filesys", "filesystem");
            _keyword = _keyword.replace("fst", "fstream");

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
                data.kalem_generated.push_str(keyword);
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
                let mut flag_name = String::new();
                let mut flag_data = String::new();

                let mut is_data: bool = false;

                for ch in variable.chars().skip(6) {
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

                if flag_name == "output" {
                    data.kalem_output = flag_data;
                }
                else if flag_name == "cpp-standard" {
                    data.kalem_cpp_standard = flag_data;
                }

                drop(flag_name);
                drop(is_data);
            }
        },
        KalemTokens::KalemLoop => {
            // 'loop' does not support arguments yet.
            data.kalem_generated.push_str(format!("{}({})",
                                                  codegen::_CPP_KALEM_LOOP,
                                                  "1").as_str());
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
        },
        KalemTokens::KalemNewline => {
            data.kalem_generated.push(codegen::NEWLINE);
        },
    }
}
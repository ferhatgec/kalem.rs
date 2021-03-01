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

    pub const _KALEM_VOID:              &str = "void";

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

    pub const _CPP_KALEM_VOID:          &str = "void";

    pub const LEFT_CURLY_BRACKET:       char = '{';
    pub const RIGHT_CURLY_BRACKET:      char = '}';

    pub const SHARP:                    char = '#';

    pub const SEMICOLON:                char = ';';
    pub const EQUAL:                    char = '=';

    pub const NEWLINE:                  char = '\n';
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

    KalemLink,

    KalemLeftCurlyBracket,
    KalemRightCurlyBracket,

    KalemNewline,
}

pub struct KalemCodegenStruct {
    pub kalem_generated: String,
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
            data.kalem_generated.push_str(format!("{} {}()",
                                                  keyword,
                                                  codegen::_CPP_KALEM_MAIN).as_str());
        },
        KalemTokens::KalemNamespace => {
            let keyword = keyword.chars().next().map(|c| &keyword[c.len_utf8()..]).unwrap();

            data.kalem_generated.push_str(format!("{} {}",
                                                  codegen::_CPP_KALEM_NAMESPACE,
                                                  keyword).as_str());
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
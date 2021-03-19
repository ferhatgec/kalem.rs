// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use std::vec;

#[allow(dead_code)]
pub enum KalemErrors {
    SyntaxError = 0,
    WarningUnsupported,
    HelpCase,
    InfoKalemRs
}

pub struct KalemErrorData<'a> {
    errors: Vec<&'a str>
}

impl KalemErrorData<'_> {
    fn initialize() -> KalemErrorData<'static> {
        let data = vec![
            // {} is formatter, but format! has does not supports
            // runtime and directly variable support,
            // it detects just literals at compile-time
            //
            // this is replacing {...} with anything you want.
            //
            //
            // {token}
            "\x1b[1;31merror\x1b[1;37m: Syntax error found at \x1b[1;36m*token*\x1b[1;37m.",

            // {type}
            "\x1b[1;93mwarning\x1b[1;37m: \x1b[1;36m*type*\x1b[1;97m is unsupported.",

            // {case} {token}
            "\x1b[0;92mhelp\x1b[1;37m: Use \x1b[1;96m*case*\x1b[1;97m instead of \x1b[1;36m*token*\x1b[1;97m.",

            // {case}
            "\x1b[0;93minfo\x1b[1;37m: Kalem.rs is not supporting \x1b[1;96m*case*\x1b[1;97m yet. \x1b[1;92mIgnored."
        ];

        KalemErrorData {
            errors: data,
        }
    }

    // An simple replace formatter implementation
    // for Kalem's error, warning, info handler.
    pub fn replformat(main_data: String, args: Vec<&str>, datas: Vec<&str>) -> String {
        if args.len() != datas.len() {
            return main_data;
        }

        let mut main_data = String::from(main_data);
        let mut i: usize = 0;

        for arg in args {
            if main_data.contains(arg) {
                main_data = main_data.replace(arg, datas[i]);
            }

            i = i + 1;
        }

        main_data
    }

    pub fn output(error: KalemErrors, file: String, line: &str, token: &str, additional: &str) {
        let x = KalemErrorData::replformat(KalemErrorData::initialize()
                                               .errors[error as usize]
                                               .to_string(),
                                            vec! {
                                                "*token*",
                                                "*type*" ,
                                                "*case*"
                                            },
                                            vec! {
                                                token,
                                                token,
                                                additional
                                            }
        );

        println!("{}\n\x1b[0;37m[\x1b[0;93m{}\x1b[1;97m] ~~> \x1b[1;37m{} : \x1b[1;97m{}\n-> {}\x1b[1;97m\n{}",
                 "-----",
                 file,
                 line,
                 token,
                 x,
                 "-----"
        );
    }
}
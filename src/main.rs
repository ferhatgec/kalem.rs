// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//


use std::{
    env,
    path::Path,
    io::prelude::*,

    fs::File,

    process::Command,
};

use std::fs;

pub mod kalem_codegen;
pub mod kalem_structure;

use kalem_codegen::{
    KalemCodegenStruct,
};

use kalem_structure::{
    read_source,
};

pub struct Kalem {
    pub kalem_filename: String,
    pub kalem_filedata: String,
}


fn init(_kalem_filename: &String) -> Kalem {
    let data = fs::read_to_string(_kalem_filename).expect("Unable to read file");

    let init = Kalem {
        kalem_filename: String::from(_kalem_filename),
        kalem_filedata: data,
    };

    return init;
}


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("{}\n{} {} [options] file...\n{}\n{}",
                 "Fegeya Kalem compiler\n",
                 "Usage:",
                 args[0],
                 "Options:",
                 "--cpp: Get generated C++ code.");

        std::process::exit(0);
    }

    // let mut file: String = String::new();

    // file = args[1].replace(".kalem", "");

    let mut option: bool= false;
    let data: Kalem;

    if &args[1] == "--cpp" && args.len() > 2 {
        data = init(&args[2]);
        option = true;
    }
    else {
        data = init(&args[1]);
    }

    let filename = data.kalem_filename.clone().replace(".kalem", ".cpp");

    let codegen: KalemCodegenStruct = read_source(data);

    let path = Path::new(&filename);

    let display = path.display();

    let mut file = match File::create(&path) {
        Err(_why) => panic!("Couldn't create {}: {}", display, _why),
        Ok(file) => file,
    };

    match file.write_all(codegen.kalem_generated.as_bytes()) {
        Err(_why) => {},
        Ok(_) => {},
    }

    let output = Command::new("clang++")
        .arg("-std=c++17")
        .arg(&filename)
        .arg("-o")
        .arg(&filename.replace(".cpp", ""))
        .output().unwrap_or_else(|e| {
        panic!("Failed to execute process: {}", e)
    });

    if !output.status.success() {
        let _s = String::from_utf8_lossy(&output.stderr);

        print!("[Error] : \n{}", _s);
    }

    if Path::new(&filename.replace(".cpp", "")).exists() && option == false {
        fs::remove_file(&filename);
    }
}

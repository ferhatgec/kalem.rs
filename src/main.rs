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

    process::Command
};

use std::fs;

pub mod kalem_codegen;
pub mod kalem_structure;
pub mod kalem_helpers;

use kalem_codegen::{
    KalemCodegenStruct,
    append_codegen
};

use kalem_structure::{
    read_source
};

use kalem_helpers::{
    extract_file_name
};

pub struct Kalem {
    pub kalem_filename: String,
    pub kalem_filedata: String,
}

const KALEM_VERSION: &str = "0.1";

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
        println!("{} - {}\n---\n{} {} [options] file...\n{}\n{}",
                 "Fegeya Kalem compiler",
                 KALEM_VERSION,
                 "Usage:",
                 args[0],
                 "Options:",
                 "--cpp: Get generated C++ code.");

        std::process::exit(0);
    }

    // let mut file: String = String::new();

    // file = args[1].replace(".kalem", "");

    let mut option: bool = false;
    let mut data: Kalem;

    if &args[1] == "--cpp" && args.len() > 2 {
        data = init(&args[2]);
        option = true;
    }
    else {
        data = init(&args[1]);
    }

    let filename = data.kalem_filename.clone().replace(".kalem", ".cpp");

    let codegen: KalemCodegenStruct = read_source(data);
    let mut temp_codegen: KalemCodegenStruct;

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

    let mut output: String = String::from(&filename.replace(".cpp", ""));

    if !codegen.kalem_output.is_empty() {
        output = codegen.kalem_output;
    }

    for i in 0..codegen.kalem_source_files.len() {
        data = init(&format!("{}.kalem", codegen.kalem_source_files[i]).to_string());

        temp_codegen = read_source(data);

        if temp_codegen.kalem_library {
            temp_codegen.kalem_generated.push_str(format!("\n#{}", append_codegen::_CPP_ENDIF).as_str());
        }

        if Path::new(format!("{}.hpp", codegen.kalem_source_files[i]).as_str()).exists() {
            fs::remove_file(format!("{}.hpp", codegen.kalem_source_files[i]).as_str())
                .expect(format!("Could not remove file '{}'", codegen.kalem_source_files[i]).as_str());
        }

        // TODO: Create simple log function implementation (success, failed, warning)

        println!("data: {}", codegen.kalem_source_files[i]);

        let mut file = match File::create(&Path::new(&format!("{}.hpp", extract_file_name(codegen.kalem_source_files[i].as_str())))) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(temp_codegen.kalem_generated.as_bytes()) {
            Err(why) => {
                panic!("Couldn't write to {}: {}",
                       format!("{}.hpp", codegen.kalem_source_files[i]),
                       why)
            },
            Ok(_) => {
                println!("Successfully wrote {} -> {}",
                         format!("{}.kalem", codegen.kalem_source_files[i]),
                         format!("{}.hpp", codegen.kalem_source_files[i]))
            },
        }
    }

    let output = Command::new(codegen.kalem_cpp_compiler)
        .arg(format!("-std={}", codegen.kalem_cpp_standard))
        .args(codegen.kalem_cpp_dirs.split(" "))
        .arg(codegen.kalem_cpp_flags)
        .arg(codegen.kalem_cpp_sysroot)
        .arg(&filename)
        .arg("-o")
        .arg(output)
        .output().unwrap_or_else(|e| {
        panic!("Failed to execute process: {}", e)
    });


    if !output.status.success() {
        let _s = String::from_utf8_lossy(&output.stderr);

        print!("[Error] : \n{}", _s);
    }

    if Path::new(&filename.replace(".cpp", "")).exists()
        && option == false
        && codegen.kalem_cpp_output == false {
        fs::remove_file(&filename)
            .expect(format!("Could not remove file '{}'", &filename).as_str());
    }

    drop(output);
    drop(filename);
}

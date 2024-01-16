#![doc = include_str!("../README.md")]

use std::{
    fs::{self, File},
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

use protobuf::{
    plugin::{CodeGeneratorRequest, CodeGeneratorResponse},
    Message, SpecialFields,
};

macro_rules! exit {
    ($($arg:tt)*) => {
        eprint!("[protoc-gen-debug] ");
        eprint!($($arg)*);
        eprintln!("");
        std::process::exit(1);
    }
}

fn main() {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).unwrap_or_else(|e| {
        exit!("Failed to read stdin: {}", e);
    });
    let request = CodeGeneratorRequest::parse_from_bytes(&buf).unwrap_or_else(|e| {
        exit!("Failed to parse CodeGeneratorRequest: {e}");
    });

    let mut path = request.parameter();
    if path.is_empty() {
        path = ".";
    }
    let path = PathBuf::from(path);

    fs::create_dir_all(&path).unwrap_or_else(|e| {
        exit!("Failed to create output directory: {e}");
    });

    File::create(path.join("code_generator_request.bin"))
        .unwrap_or_else(|e| {
            exit!("Failed to create output file: {e}");
        })
        .write_all(&buf)
        .unwrap_or_else(|e| {
            exit!("Failed to write request to disk: {e}");
        });

    File::create(path.join("code_generator_request.txt"))
        .unwrap_or_else(|e| {
            exit!("Failed to create output file: {e}");
        })
        .write_all(format!("{:#?}", request).as_bytes())
        .unwrap_or_else(|e| {
            exit!("Failed to write request to disk: {e}");
        });

    let response = CodeGeneratorResponse {
        error: None,
        supported_features: Some(1), // proto3 field presence support
        file: Vec::default(),
        special_fields: SpecialFields::default(),
    };
    let response = response.write_to_bytes().unwrap_or_else(|e| {
        exit!("Failed to serialize response: {e}");
    });
    stdout().write_all(&response).unwrap_or_else(|e| {
        exit!("Failed to write response to stdout: {e}");
    });
}

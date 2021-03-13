// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

use crate::kalem_codegen::{
    codegen,
    KalemTokens,
};

pub fn is_numeric_data(tokens: &Vec<&str>, i: usize) -> KalemTokens {
    return if tokens[i] == codegen::_KALEM_INT {
        KalemTokens::KalemInt
    }
    else {
        KalemTokens::KalemUnsigned
    };
}
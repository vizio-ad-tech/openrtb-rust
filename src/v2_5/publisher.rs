// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::category::Category;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Publisher {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cat: Vec<Category>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl Default for Publisher {
    fn default() -> Self {
        Publisher {
            id: "".to_string(),
            name: None,
            cat: vec![],
            domain: None,
            ext: None,
        }
    }
}

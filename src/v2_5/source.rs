// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fd: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pchain: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialization_skip_fields() {
        let s = Source {
            fd: None,
            tid: None,
            pchain: None,
            ext: None,
        };

        let expected = r#"{}"#;
        let serialized = serde_json::to_string(&s).unwrap();

        assert_eq!(expected, serialized)
    }
}

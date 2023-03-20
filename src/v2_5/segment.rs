// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Segment {
    // The ID that is associated with this data segment and is specific to the data provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

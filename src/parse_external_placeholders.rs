/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use std::borrow::Cow;
use std::cell::RefCell;

use rand::Rng;
use rand::XorShiftRng;

thread_local! {
    static RNG: RefCell<XorShiftRng> = RefCell::new(XorShiftRng::new_unseeded());
}

#[derive(Debug)]
pub struct RSXElementPlaceholder(Cow<'static, str>);

impl RSXElementPlaceholder {
    pub fn dummy() -> Self {
        RSXElementPlaceholder(Cow::from(""))
    }

    pub fn generate() -> Self {
        let placeholder = format!("/* rsx:{} */", RNG.with(|v| v.borrow_mut().next_u64()));
        RSXElementPlaceholder(Cow::from(placeholder))
    }
}

impl AsRef<str> for RSXElementPlaceholder {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for RSXElementPlaceholder {
    fn eq(&self, _: &RSXElementPlaceholder) -> bool {
        true
    }
}

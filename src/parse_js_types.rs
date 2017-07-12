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

use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct JSBool(pub bool);

#[derive(Debug, PartialEq)]
pub struct JSNumber(pub f64);

#[derive(Debug, PartialEq)]
pub struct JSSingleStringCharacter(pub char);

#[derive(Debug, PartialEq)]
pub struct JSSingleStringCharacters(pub String);

impl FromIterator<JSSingleStringCharacter> for JSSingleStringCharacters {
    fn from_iter<I: IntoIterator<Item = JSSingleStringCharacter>>(iter: I) -> Self {
        JSSingleStringCharacters(iter.into_iter().map(|c| c.0).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct JSDoubleStringCharacter(pub char);

#[derive(Debug, PartialEq)]
pub struct JSDoubleStringCharacters(pub String);

impl FromIterator<JSDoubleStringCharacter> for JSDoubleStringCharacters {
    fn from_iter<I: IntoIterator<Item = JSDoubleStringCharacter>>(iter: I) -> Self {
        JSDoubleStringCharacters(iter.into_iter().map(|c| c.0).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct JSIdentifierStart(pub char);

#[derive(Debug, PartialEq)]
pub struct JSIdentifierPart(pub String);

impl FromIterator<char> for JSIdentifierPart {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        JSIdentifierPart(iter.into_iter().collect())
    }
}

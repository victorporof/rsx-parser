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

use self_tokenize_macro::DefaultQuote;
use self_tokenize_trait::ToCustomTokens;

use parse_elements_types::RSXElement;
use parse_external_types::RSXParsedExpression;

#[derive(Default, Debug, PartialEq, DefaultQuote)]
pub struct RSXChildren(pub Box<[RSXChild]>);

impl From<Option<RSXChildren>> for RSXChildren {
    fn from(children: Option<RSXChildren>) -> Self {
        children.unwrap_or_default()
    }
}

impl From<Vec<RSXChild>> for RSXChildren {
    fn from(vec: Vec<RSXChild>) -> Self {
        RSXChildren(vec.into_boxed_slice())
    }
}

impl FromIterator<RSXChild> for RSXChildren {
    fn from_iter<I: IntoIterator<Item = RSXChild>>(iter: I) -> Self {
        RSXChildren::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub enum RSXChild {
    Element(RSXElement),
    Text(RSXText),
    CodeBlock(RSXParsedExpression)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXText(pub String);

impl FromIterator<RSXTextCharacter> for RSXText {
    fn from_iter<I: IntoIterator<Item = RSXTextCharacter>>(iter: I) -> Self {
        RSXText(iter.into_iter().map(|c| c.0).collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct RSXTextCharacter(pub char);

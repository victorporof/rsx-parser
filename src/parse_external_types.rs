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
use parse_external_placeholders::RSXElementPlaceholder;

#[derive(Debug, PartialEq)]
pub enum RSXRawCodeFragment {
    Empty,
    Token(char),
    Tokens(String),
    Element(RSXElement),
    ParsedExpression(RSXParsedExpression)
}

#[derive(Debug, PartialEq, DefaultQuote)]
pub struct RSXParsedExpression {
    pub tokens: String,
    pub elements: Vec<(RSXElementPlaceholder, RSXElement)>
}

impl FromIterator<RSXRawCodeFragment> for RSXParsedExpression {
    fn from_iter<I: IntoIterator<Item = RSXRawCodeFragment>>(fragments: I) -> Self {
        let mut expression = RSXParsedExpression {
            tokens: String::new(),
            elements: Vec::new()
        };
        fragments.into_iter().for_each(|fragment| match fragment {
            RSXRawCodeFragment::Empty => {}
            RSXRawCodeFragment::Token(c) => expression.tokens.push(c),
            RSXRawCodeFragment::Tokens(s) => expression.tokens.push_str(&s),
            RSXRawCodeFragment::Element(element) => {
                let placeholder = RSXElementPlaceholder::generate();
                expression.tokens.push_str(placeholder.as_ref());
                expression.elements.push((placeholder, element));
            }
            RSXRawCodeFragment::ParsedExpression(other) => {
                expression.tokens.push_str(&format!("{}{}{}", "{", other.tokens, "}"));
                expression.elements.extend(other.elements);
            }
        });
        expression
    }
}

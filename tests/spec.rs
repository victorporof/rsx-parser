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

extern crate rsx_parser;
extern crate self_tokenize_trait;

use rsx_parser::parse;
use rsx_parser::types::*;
use self_tokenize_trait::{ToCustomTokens, Tokens};

#[test]
pub fn test_simple() {
    let source = "<div>Hello world!</div>";
    let (ast, remaining): (RSXElement, _) = parse(source).unwrap();

    assert_eq!(
        (ast, remaining),
        (
            RSXElement::Normal(RSXNormalElement(
                RSXElementName::Name(RSXIdentifier("div".to_string())),
                RSXAttributes::from(vec![]),
                RSXChildren::from(vec![RSXChild::Text(RSXText("Hello world!".to_string()))])
            )),
            ""
        )
    );
}

#[test]
pub fn test_tokenize_1() {
    let source = "<foo>Hello world!</foo>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( \"foo\" ) , DOMChildren :: from ( vec ! [ \
         DOMNode :: from ( \"Hello world!\" ) , ] ) ) )"
    );
}

#[test]
pub fn test_tokenize_2() {
    let source = "<div hidden style={stylesheet.get(\".foo\")}>Hello world!</div>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( KnownElementName :: Div ) , vec ! [ \
         DOMAttribute :: from ( ( DOMAttributeName :: from ( KnownAttributeName :: Hidden ) , \
         DOMAttributeValue :: from ( true ) ) ) , DOMAttribute :: from ( ( DOMAttributeName :: \
         from ( KnownAttributeName :: Style ) , DOMAttributeValue :: from ( \
         {stylesheet.get(\".foo\")} ) ) ) , ] , vec ! [ DOMNode :: from ( \"Hello world!\" ) , ] \
         ) )",
    );
}

#[test]
pub fn test_tokenize_3() {
    let source = "<x-foo-bar>Hello world!</x-foo-bar>";
    let (ast, _): (RSXElement, _) = parse(source).unwrap();

    let mut tokens = Tokens::new();
    ast.to_custom_tokens(&mut tokens);

    assert_eq!(
        tokens.to_string(),
        "DOMNode :: from ( ( DOMTagName :: from ( \"x-foo-bar\" ) , DOMChildren :: from ( vec ! [ \
         DOMNode :: from ( \"Hello world!\" ) , ] ) ) )"
    );
}

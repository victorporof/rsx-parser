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

use combine::{ParseResult, Parser, Stream};
use combine::char::spaces;
use combine::combinator::{look_ahead, none_of, one_of, optional, parser, try, many1};

use parse_children_types::{RSXChild, RSXChildren, RSXText, RSXTextCharacter};
use parse_elements::rsx_element;
use parse_external::rsx_code_block;
use parse_js::js_whitespace;

pub fn rsx_children<I>(input: I) -> ParseResult<RSXChildren, I>
where
    I: Stream<Item = char>
{
    many1(parser(rsx_child).skip(parser(js_whitespace))).parse_stream(input)
}

pub fn rsx_child<I>(input: I) -> ParseResult<RSXChild, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_code_block).map(RSXChild::CodeBlock)),
        try(parser(rsx_element).map(RSXChild::Element)),
        parser(rsx_text).map(RSXChild::Text)
    ).parse_stream(input)
}

pub fn rsx_text<I>(input: I) -> ParseResult<RSXText, I>
where
    I: Stream<Item = char>
{
    many1(parser(rsx_text_character)).parse_stream(input)
}

pub fn rsx_text_character<I>(input: I) -> ParseResult<RSXTextCharacter, I>
where
    I: Stream<Item = char>
{
    let invalid = "{}<>";
    none_of(invalid.chars())
        .skip(optional(try(spaces().with(look_ahead(one_of(invalid.chars()))))))
        .map(RSXTextCharacter)
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    extern crate syn;

    use super::*;

    #[test]
    pub fn test_rsx_children_tokenize() {
        let value = parser(rsx_children)
            .parse(
                r#"foo
                   foo bar baz
                   123
                   <foo/>
                   {<foo/>}
                   {{1+{2+{3}}}}
                "#
            )
            .unwrap()
            .0;

        let tokens = quote! {
            vec![
                DOMNode::from("foo\n                   foo bar baz\n                   123"),
                DOMNode::from(DOMTagName::from("foo")),
                DOMNode::from({ DOMNode::from(DOMTagName::from("foo")) }),
                DOMNode::from({
                    {
                        1 + { 2 + { 3 } }
                    }
                }),
            ]
        };

        println!("{}", quote! { #value }.as_str());
        assert_eq!(syn::parse_expr(quote! { #value }.as_str()), syn::parse_expr(tokens.as_str()));
    }

    #[test]
    pub fn test_rsx_children() {
        assert_eq!(parser(rsx_children).parse("").is_err(), true);
        assert_eq!(
            parser(rsx_children).parse(" ").unwrap(),
            (RSXChildren::from(vec![" ".into()]), "")
        );
        assert_eq!(
            parser(rsx_children).parse("foo").unwrap(),
            (RSXChildren::from(vec!["foo".into()]), "")
        );
        assert_eq!(
            parser(rsx_children).parse("foo!").unwrap(),
            (RSXChildren::from(vec!["foo!".into()]), "")
        );
        assert_eq!(
            parser(rsx_children).parse("foo bar baz!").unwrap(),
            (RSXChildren::from(vec!["foo bar baz!".into()]), "")
        );
        assert_eq!(
            parser(rsx_children).parse("\"foo\" \"bar\" \"baz\"").unwrap(),
            (RSXChildren::from(vec!["\"foo\" \"bar\" \"baz\"".into()]), "")
        );
        assert_eq!(
            parser(rsx_children).parse("\"foo\"\n\"bar\"\n\"baz\"").unwrap(),
            (RSXChildren::from(vec!["\"foo\"\n\"bar\"\n\"baz\"".into()]), "")
        );
    }

    #[test]
    pub fn test_rsx_child() {
        assert_eq!(parser(rsx_child).parse("").is_err(), true);
        assert_eq!(parser(rsx_child).parse(" ").unwrap(), (" ".into(), ""));
        assert_eq!(parser(rsx_child).parse("foo").unwrap(), ("foo".into(), ""));
        assert_eq!(
            parser(rsx_child).parse("\"foo\" \"bar\" \"baz\"").unwrap(),
            ("\"foo\" \"bar\" \"baz\"".into(), "")
        );
        assert_eq!(
            parser(rsx_child).parse("\"foo\"\n\"bar\"\n\"baz\"").unwrap(),
            ("\"foo\"\n\"bar\"\n\"baz\"".into(), "")
        );
    }

    #[test]
    pub fn test_rsx_text() {
        assert_eq!(parser(rsx_text).parse("").is_err(), true);
        assert_eq!(parser(rsx_text).parse("foo < bar").unwrap(), ("foo".into(), "< bar"));
        assert_eq!(parser(rsx_text).parse("foo > bar").unwrap(), ("foo".into(), "> bar"));
        assert_eq!(parser(rsx_text).parse("foo bar baz").unwrap(), ("foo bar baz".into(), ""));
        assert_eq!(
            parser(rsx_text).parse("foo { bar } baz").unwrap(),
            ("foo".into(), "{ bar } baz")
        );
    }

    #[test]
    pub fn test_rsx_text_character() {
        assert_eq!(parser(rsx_text_character).parse("").is_err(), true);
        assert_eq!(parser(rsx_text_character).parse("{").is_err(), true);
        assert_eq!(parser(rsx_text_character).parse("}").is_err(), true);
        assert_eq!(parser(rsx_text_character).parse("<").is_err(), true);
        assert_eq!(parser(rsx_text_character).parse(">").is_err(), true);
        assert_eq!(parser(rsx_text_character).parse(" ").unwrap(), (' '.into(), ""));
        assert_eq!(parser(rsx_text_character).parse("_").unwrap(), ('_'.into(), ""));
        assert_eq!(parser(rsx_text_character).parse("$").unwrap(), ('$'.into(), ""));
        assert_eq!(parser(rsx_text_character).parse("a").unwrap(), ('a'.into(), ""));
        assert_eq!(parser(rsx_text_character).parse("0").unwrap(), ('0'.into(), ""));
    }
}

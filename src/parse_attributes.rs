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
use combine::combinator::{between, parser, token, try, many1};

use parse_attributes_types::{
    RSXAttribute,
    RSXAttributeBoolean,
    RSXAttributeName,
    RSXAttributeNumber,
    RSXAttributeString,
    RSXAttributeValue,
    RSXAttributes
};
use parse_elements::{rsx_element, rsx_identifier, rsx_namespaced_name};
use parse_external::{rsx_code_block, rsx_spread_code_block};
use parse_js::{
    js_boolean,
    js_double_string_characters,
    js_number,
    js_single_string_characters,
    js_whitespace
};

pub fn rsx_attributes<I>(input: I) -> ParseResult<RSXAttributes, I>
where
    I: Stream<Item = char>
{
    many1(parser(rsx_attribute).skip(parser(js_whitespace))).parse_stream(input)
}

pub fn rsx_attribute<I>(input: I) -> ParseResult<RSXAttribute, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_spread_attribute)),
        try(parser(rsx_custom_attribute)),
        parser(rsx_default_attribute)
    ).parse_stream(input)
}

pub fn rsx_spread_attribute<I>(input: I) -> ParseResult<RSXAttribute, I>
where
    I: Stream<Item = char>
{
    parser(rsx_spread_code_block).map(RSXAttribute::Spread).parse_stream(input)
}

pub fn rsx_custom_attribute<I>(input: I) -> ParseResult<RSXAttribute, I>
where
    I: Stream<Item = char>
{
    (
        parser(rsx_attribute_complex_name).skip(parser(js_whitespace)),
        token('=').skip(parser(js_whitespace)),
        parser(rsx_attribute_value)
    ).map(|(n, _, v)| RSXAttribute::Named(n, v))
        .parse_stream(input)
}

pub fn rsx_default_attribute<I>(input: I) -> ParseResult<RSXAttribute, I>
where
    I: Stream<Item = char>
{
    parser(rsx_attribute_complex_name)
        .map(|n| RSXAttribute::Named(n, RSXAttributeValue::Default))
        .parse_stream(input)
}

pub fn rsx_attribute_complex_name<I>(input: I) -> ParseResult<RSXAttributeName, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_namespaced_name).map(|(ns, n)| RSXAttributeName::NamedspacedName(ns, n))),
        parser(rsx_identifier).map(RSXAttributeName::Name)
    ).parse_stream(input)
}

pub fn rsx_attribute_value<I>(input: I) -> ParseResult<RSXAttributeValue, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_bracketed_attribute_bool).map(RSXAttributeValue::Boolean)),
        try(parser(rsx_bracketed_attribute_number).map(RSXAttributeValue::Number)),
        try(parser(rsx_bracketed_string_characters).map(RSXAttributeValue::Str)),
        try(parser(rsx_code_block).map(RSXAttributeValue::CodeBlock)),
        parser(rsx_element).map(RSXAttributeValue::Element)
    ).parse_stream(input)
}

pub fn rsx_bracketed_attribute_bool<I>(input: I) -> ParseResult<RSXAttributeBoolean, I>
where
    I: Stream<Item = char>
{
    choice!(
        parser(rsx_attribute_bool),
        between(
            token('{').skip(parser(js_whitespace)),
            token('}'),
            parser(rsx_attribute_bool).skip(parser(js_whitespace))
        )
    ).parse_stream(input)
}

pub fn rsx_attribute_bool<I>(input: I) -> ParseResult<RSXAttributeBoolean, I>
where
    I: Stream<Item = char>
{
    parser(js_boolean).map(RSXAttributeBoolean::from).parse_stream(input)
}

pub fn rsx_bracketed_attribute_number<I>(input: I) -> ParseResult<RSXAttributeNumber, I>
where
    I: Stream<Item = char>
{
    choice!(
        parser(rsx_attribute_number),
        between(
            token('{').skip(parser(js_whitespace)),
            token('}'),
            parser(rsx_attribute_number).skip(parser(js_whitespace))
        )
    ).parse_stream(input)
}

pub fn rsx_attribute_number<I>(input: I) -> ParseResult<RSXAttributeNumber, I>
where
    I: Stream<Item = char>
{
    parser(js_number).map(RSXAttributeNumber::from).parse_stream(input)
}

pub fn rsx_bracketed_string_characters<I>(input: I) -> ParseResult<RSXAttributeString, I>
where
    I: Stream<Item = char>
{
    choice!(
        parser(rsx_string_characters),
        between(token('{').skip(parser(js_whitespace)), token('}'), parser(rsx_string_characters))
    ).parse_stream(input)
}

pub fn rsx_string_characters<I>(input: I) -> ParseResult<RSXAttributeString, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(js_double_string_characters).map(RSXAttributeString::DoubleQuoted)),
        parser(js_single_string_characters).map(RSXAttributeString::SingleQuoted)
    ).skip(parser(js_whitespace))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    extern crate syn;

    use super::*;

    #[test]
    pub fn test_rsx_attributes_tokenize() {
        let value = parser(rsx_attributes)
            .parse(
                r#"attribute

                   attribute='c'
                   attribute="s"

                   attribute='bar'
                   attribute="bar"

                   attribute='bar"baz'
                   attribute='bar\"baz'
                   attribute='bar\'baz'
                   attribute='bar\nbaz'

                   attribute="bar'baz"
                   attribute="bar\'baz"
                   attribute="bar\"baz"
                   attribute="bar\nbaz"

                   attribute={'c'}
                   attribute={"s"}

                   attribute={"bar"}
                   attribute={'bar'}

                   attribute=true
                   attribute=false

                   attribute={true}
                   attribute={false}

                   attribute=1
                   attribute=1.2
                   attribute=1.2e3
                   attribute=1.2e-3
                   attribute=1e3
                   attribute=1e-3

                   attribute=-1
                   attribute=-1.2
                   attribute=-1.2e3
                   attribute=-1.2e-3
                   attribute=-1e3
                   attribute=-1e-3

                   attribute={1}
                   attribute={1.2}
                   attribute={1.2e3}
                   attribute={1.2e-3}
                   attribute={1e3}
                   attribute={1e-3}

                   attribute={-1}
                   attribute={-1.2}
                   attribute={-1.2e3}
                   attribute={-1.2e-3}
                   attribute={-1e3}
                   attribute={-1e-3}

                   attribute=<bar/>
                   attribute={<bar/>}

                   attribute={{'c'}}
                   attribute={{"s"}}

                   attribute={1+2+3}
                   attribute={{1+{2+{3}}}}

                   foo-bar="baz"
                   foo:bar="baz"
                   foo-a:bar-b="baz"
                "#
            )
            .unwrap()
            .0;

        let tokens = quote! {
            vec![
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(true)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("c")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("s")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar\"baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar\"baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar'baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar\nbaz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar'baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar'baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar\"baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar\nbaz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("c")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("s")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from("bar")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(true)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(false)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(true)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(false)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1.2f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1200f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(0.0012f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1000f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(0.001f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1.2f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1200f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-0.0012f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1000f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-0.001f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1.2f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1200f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(0.0012f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(1000f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(0.001f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1.2f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1200f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-0.0012f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-1000f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(-0.001f64)
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from(DOMNode::from(DOMTagName::from("bar")))
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from({ DOMNode::from(DOMTagName::from("bar")) })
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from({
                        {
                            'c'
                        }
                    })
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from({
                        {
                            "s"
                        }
                    })
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from({ 1 + 2 + 3 })
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("attribute"),
                    DOMAttributeValue::from({
                        {
                            1 + { 2 + { 3 } }
                        }
                    })
                )),
                DOMAttribute::from((
                    DOMAttributeName::from("foo-bar"),
                    DOMAttributeValue::from("baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from(("foo", "bar")),
                    DOMAttributeValue::from("baz")
                )),
                DOMAttribute::from((
                    DOMAttributeName::from(("foo-a", "bar-b")),
                    DOMAttributeValue::from("baz")
                )),
            ]
        };

        println!("{}", quote! { #value }.as_str());
        assert_eq!(syn::parse_expr(quote! { #value }.as_str()), syn::parse_expr(tokens.as_str()));
    }

    #[test]
    pub fn test_rsx_attributes() {
        assert_eq!(parser(rsx_attributes).parse("").is_err(), true);
        assert_eq!(parser(rsx_attributes).parse(" ").is_err(), true);
        assert_eq!(
            parser(rsx_attributes).parse("foo").unwrap(),
            (RSXAttributes::from(vec![("foo", "true").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo bar").unwrap(),
            (RSXAttributes::from(vec![("foo", "true").into(), ("bar", "true").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo = 'bar'").unwrap(),
            (RSXAttributes::from(vec![("foo", "bar").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo='bar' baz").unwrap(),
            (RSXAttributes::from(vec![("foo", "bar").into(), ("baz", "true").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo = 'bar' baz").unwrap(),
            (RSXAttributes::from(vec![("foo", "bar").into(), ("baz", "true").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo='bar' bar='baz'").unwrap(),
            (RSXAttributes::from(vec![("foo", "bar").into(), ("bar", "baz").into()]), "")
        );
        assert_eq!(
            parser(rsx_attributes).parse("foo = 'bar' bar='baz'").unwrap(),
            (RSXAttributes::from(vec![("foo", "bar").into(), ("bar", "baz").into()]), "")
        );
    }

    #[test]
    pub fn test_rsx_attribute() {
        assert_eq!(parser(rsx_attribute).parse("").is_err(), true);
        assert_eq!(parser(rsx_attribute).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_attribute).parse("foo").unwrap(), (("foo", "true").into(), ""));
        assert_eq!(parser(rsx_attribute).parse("foo='bar'").unwrap(), (("foo", "bar").into(), ""));
        assert_eq!(
            parser(rsx_attribute).parse("foo = 'bar'").unwrap(),
            (("foo", "bar").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo-bar='baz'").unwrap(),
            (("foo-bar", "baz").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo-bar = 'baz'").unwrap(),
            (("foo-bar", "baz").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo:bar='baz'").unwrap(),
            ((("foo", "bar"), "baz").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo:bar = 'baz'").unwrap(),
            ((("foo", "bar"), "baz").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo.bar='baz'").unwrap(),
            (("foo", "true").into(), ".bar='baz'")
        );
        assert_eq!(
            parser(rsx_attribute).parse("foo.bar = 'baz'").unwrap(),
            (("foo", "true").into(), ".bar = 'baz'")
        );
    }

    #[test]
    pub fn test_rsx_attribute_name() {
        assert_eq!(parser(rsx_attribute_complex_name).parse("").is_err(), true);
        assert_eq!(parser(rsx_attribute_complex_name).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_attribute_complex_name).parse("foo").unwrap(), ("foo".into(), ""));
        assert_eq!(
            parser(rsx_attribute_complex_name).parse("foo-bar").unwrap(),
            ("foo-bar".into(), "")
        );
        assert_eq!(
            parser(rsx_attribute_complex_name).parse("foo:bar").unwrap(),
            (("foo", "bar").into(), "")
        );
        assert_eq!(
            parser(rsx_attribute_complex_name).parse("foo.bar").unwrap(),
            ("foo".into(), ".bar")
        );
    }

    #[test]
    pub fn test_rsx_attribute_value() {
        assert_eq!(parser(rsx_attribute_value).parse("").is_err(), true);
        assert_eq!(parser(rsx_attribute_value).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_attribute_value).parse(r#""""#).unwrap(), (("", '\'').into(), ""));
        assert_eq!(parser(rsx_attribute_value).parse(r#"" ""#).unwrap(), ((" ", '\'').into(), ""));
        assert_eq!(
            parser(rsx_attribute_value).parse(r#""foo""#).unwrap(),
            (("foo", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_attribute_value).parse(r#"'bar'"#).unwrap(),
            (("bar", '"').into(), "")
        );
    }

    #[test]
    pub fn test_rsx_string_characters() {
        assert_eq!(parser(rsx_string_characters).parse("").is_err(), true);
        assert_eq!(parser(rsx_string_characters).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_string_characters).parse(r#""""#).unwrap(), (("", '\'').into(), ""));
        assert_eq!(
            parser(rsx_string_characters).parse(r#"" ""#).unwrap(),
            ((" ", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#""foo""#).unwrap(),
            (("foo", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#"'bar'"#).unwrap(),
            (("bar", '"').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#""foo'bar""#).unwrap(),
            (("foo'bar", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#"'foo"bar'"#).unwrap(),
            ((r#"foo"bar"#, '"').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#""foo\'bar""#).unwrap(),
            (("foo'bar", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#"'foo\"bar'"#).unwrap(),
            ((r#"foo"bar"#, '"').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#""foo\"bar""#).unwrap(),
            ((r#"foo"bar"#, '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#"'foo\'bar'"#).unwrap(),
            (("foo'bar", '"').into(), "")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#""foo"bar""#).unwrap(),
            (("foo", '\'').into(), "bar\"")
        );
        assert_eq!(
            parser(rsx_string_characters).parse(r#"'foo'bar'"#).unwrap(),
            (("foo", '"').into(), "bar\'")
        );
    }

    #[test]
    pub fn test_rsx_bracketed_string_characters() {
        assert_eq!(parser(rsx_bracketed_string_characters).parse("").is_err(), true);
        assert_eq!(parser(rsx_bracketed_string_characters).parse(" ").is_err(), true);
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{""}"#).unwrap(),
            (("", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{" "}"#).unwrap(),
            ((" ", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{"{{{}}}"}"#).unwrap(),
            (("{{{}}}", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{"foo"}"#).unwrap(),
            (("foo", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{'bar'}"#).unwrap(),
            (("bar", '"').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{"foo'bar"}"#).unwrap(),
            (("foo'bar", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{'foo"bar'}"#).unwrap(),
            ((r#"foo"bar"#, '"').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{"foo\'bar"}"#).unwrap(),
            (("foo'bar", '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{'foo\"bar'}"#).unwrap(),
            ((r#"foo"bar"#, '"').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{"foo\"bar"}"#).unwrap(),
            ((r#"foo"bar"#, '\'').into(), "")
        );
        assert_eq!(
            parser(rsx_bracketed_string_characters).parse(r#"{'foo\'bar'}"#).unwrap(),
            (("foo'bar", '"').into(), "")
        );
        assert_eq!(parser(rsx_bracketed_string_characters).parse(r#"{"foo"bar"}"#).is_err(), true);
        assert_eq!(parser(rsx_bracketed_string_characters).parse(r#"{'foo'bar'}"#).is_err(), true);
    }
}

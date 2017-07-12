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
use combine::combinator::{env_parser, look_ahead, optional, parser, token, tokens, try, sep_by1};
use itertools::Itertools;

use parse_attributes::rsx_attributes;
use parse_children::rsx_children;
use parse_elements_types::{
    RSXClosingElement,
    RSXElement,
    RSXElementName,
    RSXIdentifier,
    RSXNormalElement,
    RSXOpeningElement,
    RSXSelfClosingElement
};
use parse_js::{js_identifier_part, js_identifier_start, js_whitespace};
use parse_misc::{close_tag, closing_element_open_tag, open_tag, self_closing_element_close_tag};

pub fn rsx_element<I>(input: I) -> ParseResult<RSXElement, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_self_closing_element).map(RSXElement::SelfClosing)),
        parser(rsx_normal_element).map(RSXElement::Normal)
    ).parse_stream(input)
}

pub fn rsx_element_open<I>(input: I) -> ParseResult<RSXElementName, I>
where
    I: Stream<Item = char>
{
    parser(open_tag).skip(parser(js_whitespace)).with(parser(rsx_element_name)).parse_stream(input)
}

pub fn rsx_self_closing_element<I>(input: I) -> ParseResult<RSXSelfClosingElement, I>
where
    I: Stream<Item = char>
{
    (
        parser(rsx_element_open).skip(parser(js_whitespace)),
        optional(parser(rsx_attributes).skip(parser(js_whitespace))),
        parser(self_closing_element_close_tag)
    ).map(|(n, a, _)| RSXSelfClosingElement(n, a.into()))
        .parse_stream(input)
}

pub fn rsx_normal_element<I>(input: I) -> ParseResult<RSXNormalElement, I>
where
    I: Stream<Item = char>
{
    look_ahead(parser(rsx_element_open)).parse_stream(input).and_then(|(name, consumed)| {
        (
            parser(rsx_opening_element).skip(parser(js_whitespace)),
            optional(parser(rsx_children).skip(parser(js_whitespace))),
            env_parser(&name, rsx_closing_element)
        ).map(|(RSXOpeningElement(n, a), c, _)| RSXNormalElement(n, a, c.into()))
            .parse_stream(consumed.into_inner())
    })
}

pub fn rsx_opening_element<I>(input: I) -> ParseResult<RSXOpeningElement, I>
where
    I: Stream<Item = char>
{
    (
        parser(rsx_element_open).skip(parser(js_whitespace)),
        optional(parser(rsx_attributes).skip(parser(js_whitespace))),
        parser(close_tag)
    ).map(|(n, a, _)| RSXOpeningElement(n, a.into()))
        .parse_stream(input)
}

pub fn rsx_closing_element<I>(name: &RSXElementName, input: I) -> ParseResult<RSXClosingElement, I>
where
    I: Stream<Item = char>
{
    let chars = name.to_string();
    let chars_ws = chars.split('-').join(" - ");
    let expected = chars.clone().into();
    let expected_ws = chars_ws.clone().into();
    let cmp_ignore_ws = |l: char, r: char| l == r || l.is_whitespace() && r.is_whitespace();
    (
        parser(closing_element_open_tag).skip(parser(js_whitespace)),
        choice!(
            try(tokens(&cmp_ignore_ws, expected_ws, chars_ws.chars()).skip(parser(js_whitespace))),
            tokens(&cmp_ignore_ws, expected, chars.chars()).skip(parser(js_whitespace))
        ),
        parser(close_tag)
    ).map(|_| RSXClosingElement(name))
        .parse_stream(input)
}

pub fn rsx_element_name<I>(input: I) -> ParseResult<RSXElementName, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_member_expression).map(RSXElementName::MemberExpression)),
        try(parser(rsx_namespaced_name).map(|(ns, n)| RSXElementName::NamedspacedName(ns, n))),
        parser(rsx_identifier).map(RSXElementName::Name)
    ).parse_stream(input)
}

pub fn rsx_identifier_simple<I>(input: I) -> ParseResult<RSXIdentifier, I>
where
    I: Stream<Item = char>
{
    (try(parser(js_identifier_start)), optional(parser(js_identifier_part)))
        .map(|(s, p)| p.map(|p| format!("{}{}", s.0, p.0)).unwrap_or_else(|| format!("{}", s.0)))
        .map(RSXIdentifier)
        .parse_stream(input)
}

pub fn rsx_identifier<I>(input: I) -> ParseResult<RSXIdentifier, I>
where
    I: Stream<Item = char>
{
    sep_by1(
        parser(rsx_identifier_simple).skip(parser(js_whitespace)),
        token('-').skip(parser(js_whitespace))
    ).parse_stream(input)
}

pub fn rsx_namespaced_name<I>(input: I) -> ParseResult<(RSXIdentifier, RSXIdentifier), I>
where
    I: Stream<Item = char>
{
    (parser(rsx_identifier), token(':'), parser(rsx_identifier))
        .map(|(l, _, r)| (l, r))
        .parse_stream(input)
}

pub fn rsx_member_expression<I>(input: I) -> ParseResult<Box<[RSXIdentifier]>, I>
where
    I: Stream<Item = char>
{
    (parser(rsx_identifier), token('.'), sep_by1(parser(rsx_identifier), token('.')))
        .map(|(i, _, mut v): (_, _, Vec<_>)| {
            v.insert(0, i);
            v.into_boxed_slice()
        })
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    extern crate syn;

    use super::*;
    use super::env_parser as env_p;
    use super::parser as p;

    use parse_attributes_types::{RSXAttribute, RSXAttributeName, RSXAttributeValue, RSXAttributes};
    use parse_children_types::{RSXChild, RSXChildren, RSXText};

    #[test]
    pub fn test_rsx_element_tokenize() {
        let value = parser(rsx_element)
            .parse(
                r#"<root>
                     <foo/>
                     <foo.member.bar/>
                     <foo-bar/>
                     <foo - bar/>
                     <foo-bar.member/>
                     <foo - bar.member/>
                     <foo-bar.member.bar-baz/>
                     <foo - bar.member.bar - baz/>
                     <foo:bar/>
                     <foo-a:bar-b/>
                     <foo-a:bar-b/>
                     <foo></foo>
                     <x - foo - bar></x - foo - bar>
                   </root>
                "#
            )
            .unwrap()
            .0;

        let tokens = quote! {
            DOMNode::from((
                DOMTagName::from("root"),
                DOMChildren::from(vec![
                    DOMNode::from(DOMTagName::from("foo")),
                    DOMNode::from(DOMTagName::from(box ["foo", "member", "bar",])),
                    DOMNode::from(DOMTagName::from("foo-bar")),
                    DOMNode::from(DOMTagName::from("foo-bar")),
                    DOMNode::from(DOMTagName::from(box ["foo-bar", "member",])),
                    DOMNode::from(DOMTagName::from(box ["foo-bar", "member",])),
                    DOMNode::from(DOMTagName::from(box ["foo-bar", "member", "bar-baz",])),
                    DOMNode::from(DOMTagName::from(box ["foo-bar", "member", "bar-baz",])),
                    DOMNode::from(DOMTagName::from(("foo", "bar"))),
                    DOMNode::from(DOMTagName::from(("foo-a", "bar-b"))),
                    DOMNode::from(DOMTagName::from(("foo-a", "bar-b"))),
                    DOMNode::from(DOMTagName::from("foo")),
                    DOMNode::from(DOMTagName::from("x-foo-bar")),
                ])
            ))
        };

        println!("{}", quote! { #value }.as_str());
        assert_eq!(syn::parse_expr(quote! { #value }.as_str()), syn::parse_expr(tokens.as_str()));
    }

    #[test]
    pub fn test_rsx_element() {
        assert_eq!(
            parser(rsx_element)
                .parse(
                    r#"<foo bar='baz'>
                            <foo bar='baz'>
                                <foo bar='baz'/>
                                hello<bar/>
                                world<baz/>
                            </foo>
                        </foo>"#
                )
                .unwrap(),
            (
                RSXElement::Normal(RSXNormalElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Str("baz".into())
                        ),
                    ]),
                    RSXChildren::from(vec![
                        RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name("foo".into()),
                            RSXAttributes::from(vec![
                                RSXAttribute::Named(
                                    RSXAttributeName::Name("bar".into()),
                                    RSXAttributeValue::Str("baz".into())
                                ),
                            ]),
                            RSXChildren::from(vec![
                                RSXChild::Element(RSXElement::SelfClosing(RSXSelfClosingElement(
                                    RSXElementName::Name("foo".into()),
                                    RSXAttributes::from(vec![
                                        RSXAttribute::Named(
                                            RSXAttributeName::Name("bar".into()),
                                            RSXAttributeValue::Str("baz".into())
                                        ),
                                    ])
                                ))),
                                RSXChild::Text(RSXText("hello".into())),
                                RSXChild::Element(RSXElement::SelfClosing(RSXSelfClosingElement(
                                    RSXElementName::Name("bar".into()),
                                    RSXAttributes::from(vec![])
                                ))),
                                RSXChild::Text(RSXText("world".into())),
                                RSXChild::Element(RSXElement::SelfClosing(RSXSelfClosingElement(
                                    RSXElementName::Name("baz".into()),
                                    RSXAttributes::from(vec![])
                                ))),
                            ])
                        ))),
                    ])
                )),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_normal_element() {
        assert_eq!(parser(rsx_normal_element).parse("").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo>").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo/>").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("< foo />").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo/ >").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("</foo>").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo-bar/>").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo:bar/>").is_err(), true);
        assert_eq!(parser(rsx_normal_element).parse("<foo.bar/>").is_err(), true);

        assert_eq!(parser(rsx_normal_element).parse("<foo></foo>").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_normal_element).parse("< foo ></foo>").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_normal_element).parse("<foo></ foo >").unwrap(), ("foo".into(), ""));
        assert_eq!(
            parser(rsx_normal_element).parse("< foo ></ foo >").unwrap(),
            ("foo".into(), "")
        );

        assert_eq!(
            parser(rsx_normal_element).parse("<foo-bar></foo-bar>").unwrap(),
            ("foo-bar".into(), "")
        );
        assert_eq!(
            parser(rsx_normal_element).parse("<foo:bar></foo:bar>").unwrap(),
            (("foo", "bar").into(), "")
        );
        assert_eq!(
            parser(rsx_normal_element).parse("<foo.bar></foo.bar>").unwrap(),
            (vec!["foo", "bar"][..].into(), "")
        );
    }

    #[test]
    pub fn test_rsx_normal_element_with_attributes() {
        assert_eq!(
            parser(rsx_normal_element).parse("<foo bar></foo>").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_normal_element).parse("< foo bar ></ foo >").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_normal_element).parse("<foo bar baz></foo>").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_normal_element).parse("< foo bar baz ></ foo >").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_normal_element).parse("<f b='z'></f>").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_normal_element).parse("< f b = 'z' ></ f >").unwrap(),
            (
                RSXNormalElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ]),
                    RSXChildren::from(vec![])
                ),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_self_closing_element() {
        assert_eq!(parser(rsx_self_closing_element).parse("").is_err(), true);
        assert_eq!(parser(rsx_self_closing_element).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_self_closing_element).parse("<foo>").is_err(), true);
        assert_eq!(parser(rsx_self_closing_element).parse("<foo/>").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_self_closing_element).parse("< foo />").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_self_closing_element).parse("<foo/ >").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_self_closing_element).parse("</foo>").is_err(), true);
        assert_eq!(
            parser(rsx_self_closing_element).parse("<foo-bar/>").unwrap(),
            ("foo-bar".into(), "")
        );
        assert_eq!(
            parser(rsx_self_closing_element).parse("<foo:bar/>").unwrap(),
            (("foo", "bar").into(), "")
        );
        assert_eq!(
            parser(rsx_self_closing_element).parse("<foo.bar/>").unwrap(),
            (vec!["foo", "bar"][..].into(), "")
        );
    }

    #[test]
    pub fn test_rsx_self_closing_element_with_attributes() {
        assert_eq!(
            parser(rsx_self_closing_element).parse("<foo bar/>").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_self_closing_element).parse("< foo bar />").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_self_closing_element).parse("<foo bar baz/>").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_self_closing_element).parse("< foo bar baz />").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_self_closing_element).parse("<f b='z'/>").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_self_closing_element).parse("< f b = 'z' />").unwrap(),
            (
                RSXSelfClosingElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ])
                ),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_opening_element() {
        assert_eq!(parser(rsx_opening_element).parse("").is_err(), true);
        assert_eq!(parser(rsx_opening_element).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_opening_element).parse("<foo>").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_opening_element).parse("< foo >").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_opening_element).parse("<foo/>").is_err(), true);
        assert_eq!(parser(rsx_opening_element).parse("</foo>").is_err(), true);
        assert_eq!(parser(rsx_opening_element).parse("<foo-bar>").unwrap(), ("foo-bar".into(), ""));
        assert_eq!(
            parser(rsx_opening_element).parse("<foo:bar>").unwrap(),
            (("foo", "bar").into(), "")
        );
        assert_eq!(
            parser(rsx_opening_element).parse("<foo.bar>").unwrap(),
            (vec!["foo", "bar"][..].into(), "")
        );
    }

    #[test]
    pub fn test_rsx_opening_element_with_attributes() {
        assert_eq!(
            parser(rsx_opening_element).parse("<foo bar>").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_opening_element).parse("< foo bar >").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_opening_element).parse("<foo bar baz>").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_opening_element).parse("< foo bar baz >").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("foo".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("bar".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("baz".into()),
                            RSXAttributeValue::Default
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_opening_element).parse("<f b='z'>").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ])
                ),
                ""
            )
        );

        assert_eq!(
            parser(rsx_opening_element).parse("< f b = 'z' >").unwrap(),
            (
                RSXOpeningElement(
                    RSXElementName::Name("f".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("b".into()),
                            RSXAttributeValue::Str("z".into())
                        ),
                    ])
                ),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_closing_element() {
        assert_eq!(env_p(&"".into(), rsx_closing_element).parse("").is_err(), true);
        assert_eq!(env_p(&"".into(), rsx_closing_element).parse(" ").is_err(), true);
        assert_eq!(env_p(&"foo".into(), rsx_closing_element).parse("<foo>").is_err(), true);
        assert_eq!(env_p(&"foo".into(), rsx_closing_element).parse("<foo/>").is_err(), true);
        assert_eq!(env_p(&"foo".into(), rsx_closing_element).parse("</foo>").is_err(), false);
        assert_eq!(
            env_p(&"baz".into(), rsx_closing_element)
                .parse("</foo>")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `f`\nExpected `baz`\n"),
            true
        );
        assert_eq!(env_p(&"foo".into(), rsx_closing_element).parse("</ foo >").is_err(), false);
        assert_eq!(
            env_p(&"baz".into(), rsx_closing_element)
                .parse("</ foo >")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `f`\nExpected `baz`\n"),
            true
        );
        assert_eq!(env_p(&"foo".into(), rsx_closing_element).parse("< /foo>").is_err(), false);
        assert_eq!(
            env_p(&"baz".into(), rsx_closing_element)
                .parse("< /foo>")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `f`\nExpected `baz`\n"),
            true
        );
        assert_eq!(
            env_p(&"foo-bar".into(), rsx_closing_element).parse("</foo-bar>").is_err(),
            false
        );
        assert_eq!(
            env_p(&"foo-baz".into(), rsx_closing_element)
                .parse("</foo-bar>")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `r`\nExpected `foo-baz`\n"),
            true
        );
        assert_eq!(
            env_p(&"foo:bar".into(), rsx_closing_element).parse("</foo:bar>").is_err(),
            false
        );
        assert_eq!(
            env_p(&"foo:baz".into(), rsx_closing_element)
                .parse("</foo:bar>")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `r`\nExpected `foo:baz`\n"),
            true
        );
        assert_eq!(
            env_p(&"foo.bar".into(), rsx_closing_element).parse("</foo.bar>").is_err(),
            false
        );
        assert_eq!(
            env_p(&"foo.baz".into(), rsx_closing_element)
                .parse("</foo.bar>")
                .err()
                .unwrap()
                .to_string()
                .contains("Unexpected `r`\nExpected `foo.baz`\n"),
            true
        );
    }

    #[test]
    pub fn test_rsx_element_name() {
        assert_eq!(parser(rsx_element_name).parse("").is_err(), true);
        assert_eq!(parser(rsx_element_name).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_element_name).parse("foo").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_element_name).parse("foo-bar").unwrap(), ("foo-bar".into(), ""));
        assert_eq!(parser(rsx_element_name).parse("foo:bar").unwrap(), (("foo", "bar").into(), ""));
        assert_eq!(
            p(rsx_element_name).parse("foo.bar").unwrap(),
            (vec!["foo", "bar"][..].into(), "")
        );
    }

    #[test]
    pub fn test_rsx_identifier_simple() {
        assert_eq!(parser(rsx_identifier_simple).parse("").is_err(), true);
        assert_eq!(parser(rsx_identifier_simple).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_identifier_simple).parse("foo").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_identifier_simple).parse("foo_bar").unwrap(), ("foo_bar".into(), ""));
        assert_eq!(parser(rsx_identifier_simple).parse("foo$bar").unwrap(), ("foo$bar".into(), ""));
        assert_eq!(parser(rsx_identifier_simple).parse("1foo").is_err(), true);
        assert_eq!(parser(rsx_identifier_simple).parse("foo1").unwrap(), ("foo1".into(), ""));
    }

    #[test]
    pub fn test_rsx_identifier() {
        assert_eq!(parser(rsx_identifier).parse("").is_err(), true);
        assert_eq!(parser(rsx_identifier).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_identifier).parse("foo").unwrap(), ("foo".into(), ""));
        assert_eq!(parser(rsx_identifier).parse("foo-bar").unwrap(), ("foo-bar".into(), ""));
        assert_eq!(parser(rsx_identifier).parse("$foo-$bar").unwrap(), ("$foo-$bar".into(), ""));
        assert_eq!(parser(rsx_identifier).parse("1foo-bar").is_err(), true);
        assert_eq!(parser(rsx_identifier).parse("foo1-bar").unwrap(), ("foo1-bar".into(), ""));
        assert_eq!(parser(rsx_identifier).parse("foo-1bar").is_err(), true);
        assert_eq!(parser(rsx_identifier).parse("foo-bar1").unwrap(), ("foo-bar1".into(), ""));
    }

    #[test]
    pub fn test_rsx_namespaced_name() {
        assert_eq!(parser(rsx_namespaced_name).parse("").is_err(), true);
        assert_eq!(parser(rsx_namespaced_name).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_namespaced_name).parse("foo").is_err(), true);
        assert_eq!(
            p(rsx_namespaced_name).parse("foo:bar").unwrap(),
            (("foo".into(), "bar".into()), "")
        );
        assert_eq!(
            p(rsx_namespaced_name).parse("$foo:$bar").unwrap(),
            (("$foo".into(), "$bar".into()), "")
        );
        assert_eq!(
            p(rsx_namespaced_name).parse("foo:bar:baz").unwrap(),
            (("foo".into(), "bar".into()), ":baz")
        );
        assert_eq!(
            p(rsx_namespaced_name).parse("$foo:$bar:$baz").unwrap(),
            (("$foo".into(), "$bar".into()), ":$baz")
        );
        assert_eq!(parser(rsx_namespaced_name).parse("1foo:bar").is_err(), true);
        assert_eq!(
            p(rsx_namespaced_name).parse("foo1:bar").unwrap(),
            (("foo1".into(), "bar".into()), "")
        );
        assert_eq!(parser(rsx_namespaced_name).parse("foo:1bar").is_err(), true);
        assert_eq!(
            p(rsx_namespaced_name).parse("foo:bar1").unwrap(),
            (("foo".into(), "bar1".into()), "")
        );
    }

    #[test]
    pub fn test_rsx_member_expression() {
        assert_eq!(parser(rsx_member_expression).parse("").is_err(), true);
        assert_eq!(parser(rsx_member_expression).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_member_expression).parse("foo").is_err(), true);
        assert_eq!(
            p(rsx_member_expression).parse("foo.bar").unwrap().0.into_vec(),
            vec!["foo".into(), "bar".into()]
        );
        assert_eq!(
            p(rsx_member_expression).parse("$foo.$bar").unwrap().0.into_vec(),
            vec!["$foo".into(), "$bar".into()]
        );
        assert_eq!(
            p(rsx_member_expression).parse("foo.bar.baz").unwrap().0.into_vec(),
            vec!["foo".into(), "bar".into(), "baz".into()]
        );
        assert_eq!(
            p(rsx_member_expression).parse("$foo.$bar.$baz").unwrap().0.into_vec(),
            vec!["$foo".into(), "$bar".into(), "$baz".into()]
        );
        assert_eq!(parser(rsx_member_expression).parse("1foo.bar").is_err(), true);
        assert_eq!(
            p(rsx_member_expression).parse("foo1.bar").unwrap().0.into_vec(),
            vec!["foo1".into(), "bar".into()]
        );
        assert_eq!(parser(rsx_member_expression).parse("foo.1bar").is_err(), true);
        assert_eq!(
            p(rsx_member_expression).parse("foo.bar1").unwrap().0.into_vec(),
            vec!["foo".into(), "bar1".into()]
        );
    }
}

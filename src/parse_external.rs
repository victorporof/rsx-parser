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
use combine::char::string;
use combine::combinator::{between, many, none_of, optional, parser, token, try, value, many1};

use parse_elements::rsx_element;
use parse_external_types::{RSXParsedExpression, RSXRawCodeFragment};
use parse_rust::{rs_char, rs_comment, rs_string, rs_whitespace};

pub fn rsx_code_block_begin<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    token('{').with(value(())).parse_stream(input)
}

pub fn rsx_code_block_end<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    token('}').with(value(())).parse_stream(input)
}

pub fn rsx_code_block<I>(input: I) -> ParseResult<RSXParsedExpression, I>
where
    I: Stream<Item = char>
{
    between(
        parser(rsx_code_block_begin),
        parser(rsx_code_block_end),
        many(parser(rsx_code_block_fragment))
    ).parse_stream(input)
}

pub fn rsx_spread_code_block<I>(input: I) -> ParseResult<RSXParsedExpression, I>
where
    I: Stream<Item = char>
{
    between(
        parser(rsx_code_block_begin),
        parser(rsx_code_block_end),
        (optional(parser(rs_whitespace)), string("..."))
            .with(many1(parser(rsx_code_block_fragment)))
    ).parse_stream(input)
}

pub fn rsx_code_block_fragment<I>(input: I) -> ParseResult<RSXRawCodeFragment, I>
where
    I: Stream<Item = char>
{
    choice!(
        try(parser(rsx_code_block).map(RSXRawCodeFragment::ParsedExpression)),
        try(parser(rsx_element).map(RSXRawCodeFragment::Element)),
        try(parser(rs_comment).map(|_| RSXRawCodeFragment::Empty)),
        try(parser(rs_char).map(|v| RSXRawCodeFragment::Tokens(format!("'{}'", v.0)))),
        try(parser(rs_string).map(|v| RSXRawCodeFragment::Tokens(format!("\"{}\"", v.0)))),
        none_of("}".chars()).map(RSXRawCodeFragment::Token)
    ).parse_stream(input)
}

#[cfg(test)]
mod tests {
    extern crate syn;

    use super::*;

    use parse_attributes_types::RSXAttributes;
    use parse_children_types::{RSXChild, RSXChildren};
    use parse_elements_types::{RSXElement, RSXElementName, RSXNormalElement, RSXSelfClosingElement};
    use parse_external_placeholders::RSXElementPlaceholder;
    use parse_external_types::RSXParsedExpression;

    #[test]
    pub fn test_rsx_code_block_tokenize() {
        let value = parser(rsx_code_block)
            .parse(
                r#"{
                    if foo {
                        <bar { ...props }>
                            hello
                            {
                                name(
                                    <world/>
                                )
                            }
                        </bar>
                    } else {
                        <baz { ...props }>
                            goodbye
                            {
                                name(
                                    <blue-sky/>
                                )
                            }
                        </baz>
                    }
                    <what { ...
                        if happy_and_you_know_it {
                            <clap { ...props }/>
                        } else {
                            <dont.clap { ...props }/>
                        }
                    }/>
                }"#
            )
            .unwrap()
            .0;

        let tokens = quote! {
            {
                if foo {
                    DOMNode::from((
                        DOMTagName::from("bar"),
                        vec![DOMAttribute::from({ props }),],
                        vec![
                            DOMNode::from("hello"),
                            DOMNode::from({ name(DOMNode::from(DOMTagName::from("world"))) }),
                        ]
                    ))
                } else {
                    DOMNode::from((
                        DOMTagName::from("baz"),
                        vec![DOMAttribute::from({ props }),],
                        vec![
                            DOMNode::from("goodbye"),
                            DOMNode::from({ name(DOMNode::from(DOMTagName::from("blue-sky"))) }),
                        ]
                    ))
                }
                DOMNode::from((
                    DOMTagName::from("what"),
                    DOMAttributes::from(vec![
                        DOMAttribute::from({
                            if happy_and_you_know_it {
                                DOMNode::from((
                                    DOMTagName::from("clap"),
                                    DOMAttributes::from(vec![DOMAttribute::from({ props }),])
                                ))
                            } else {
                                DOMNode::from((
                                    DOMTagName::from(box ["dont", "clap",]),
                                    DOMAttributes::from(vec![DOMAttribute::from({ props }),])
                                ))
                            }
                        }),
                    ])
                ))
            }
        };

        println!("{}", quote! { #value }.as_str());
        assert_eq!(syn::parse_expr(quote! { #value }.as_str()), syn::parse_expr(tokens.as_str()));
    }

    #[test]
    pub fn test_rsx_code_block() {
        assert_eq!(parser(rsx_code_block).parse("").is_err(), true);
        assert_eq!(parser(rsx_code_block).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_code_block).parse("foo").is_err(), true);
        assert_eq!(
            parser(rsx_code_block).parse("{}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{{}}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "{}".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{foo}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "foo".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ foo }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " foo ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{{foo}}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "{foo}".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ { foo } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " { foo } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ foo bar baz }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " foo bar baz ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ if foo { bar } else { baz } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { bar } else { baz } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ if foo { \"bar\" } else { \"baz\" } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { \"bar\" } else { \"baz\" } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ if foo { \"{bar}\" } else { \"{baz}\" } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { \"{bar}\" } else { \"{baz}\" } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block)
                .parse("{ if foo { /* {bar} */ } else { /* {baz} */ } }")
                .unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo {  } else {  } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block).parse("{ if foo { <bar/> } else { <baz/> } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { /* rsx:15848556381555908996 */ } else { /* \
                             rsx:13987966085338848396 */ } "
                        .into(),
                    elements: vec![
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::SelfClosing(RSXSelfClosingElement(
                                RSXElementName::Name("bar".into()),
                                RSXAttributes::from(vec![])
                            ))
                        ),
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::SelfClosing(RSXSelfClosingElement(
                                RSXElementName::Name("baz".into()),
                                RSXAttributes::from(vec![])
                            ))
                        ),
                    ]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block)
                .parse("{ if foo { <bar>{ hello }</bar> } else { <baz>{ world }</baz> } }")
                .unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { /* rsx:6801744144136471498 */ } else { /* \
                             rsx:4005265977620077421 */ } "
                        .into(),
                    elements: vec![
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("bar".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " hello ".into(),
                                        elements: vec![]
                                    }),
                                ])
                            ))
                        ),
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("baz".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " world ".into(),
                                        elements: vec![]
                                    }),
                                ])
                            ))
                        ),
                    ]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block)
                .parse("{ if foo { <bar>{ <hello/> }</bar> } else { <baz>{ <world/> }</baz> } }")
                .unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { /* rsx:13195988079190323012 */ } else { /* \
                             rsx:5146857450694500275 */ } "
                        .into(),
                    elements: vec![
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("bar".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " /* rsx:16730135874920933484 */ ".into(),
                                        elements: vec![
                                            (
                                                RSXElementPlaceholder::dummy(),
                                                RSXElement::SelfClosing(RSXSelfClosingElement(
                                                    RSXElementName::Name("hello".into()),
                                                    RSXAttributes::from(vec![])
                                                ))
                                            ),
                                        ]
                                    }),
                                ])
                            ))
                        ),
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("baz".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " /* rsx:11802923454833793349 */ ".into(),
                                        elements: vec![
                                            (
                                                RSXElementPlaceholder::dummy(),
                                                RSXElement::SelfClosing(RSXSelfClosingElement(
                                                    RSXElementName::Name("world".into()),
                                                    RSXAttributes::from(vec![])
                                                ))
                                            ),
                                        ]
                                    }),
                                ])
                            ))
                        ),
                    ]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_code_block)
                .parse(
                    "{ if foo { <bar>{ lorem(<hello/>) }</bar> } else { <baz>{ ipsum(<world/>) \
                     }</baz> } }"
                )
                .unwrap(),
            (
                RSXParsedExpression {
                    tokens: " if foo { /* rsx:17291863876572781215 */ } else { /* \
                             rsx:10717465471001048421 */ } "
                        .into(),
                    elements: vec![
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("bar".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " lorem(/* rsx:14589965171469706430 */) ".into(),
                                        elements: vec![
                                            (
                                                RSXElementPlaceholder::dummy(),
                                                RSXElement::SelfClosing(RSXSelfClosingElement(
                                                    RSXElementName::Name("hello".into()),
                                                    RSXAttributes::from(vec![])
                                                ))
                                            ),
                                        ]
                                    }),
                                ])
                            ))
                        ),
                        (
                            RSXElementPlaceholder::dummy(),
                            RSXElement::Normal(RSXNormalElement(
                                RSXElementName::Name("baz".into()),
                                RSXAttributes::from(vec![]),
                                RSXChildren::from(vec![
                                    RSXChild::CodeBlock(RSXParsedExpression {
                                        tokens: " ipsum(/* rsx:6790293794189608791 */) ".into(),
                                        elements: vec![
                                            (
                                                RSXElementPlaceholder::dummy(),
                                                RSXElement::SelfClosing(RSXSelfClosingElement(
                                                    RSXElementName::Name("world".into()),
                                                    RSXAttributes::from(vec![])
                                                ))
                                            ),
                                        ]
                                    }),
                                ])
                            ))
                        ),
                    ]
                },
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_spread_code_block() {
        assert_eq!(parser(rsx_spread_code_block).parse("").is_err(), true);
        assert_eq!(parser(rsx_spread_code_block).parse(" ").is_err(), true);
        assert_eq!(parser(rsx_spread_code_block).parse("foo").is_err(), true);
        assert_eq!(parser(rsx_spread_code_block).parse("{...}").is_err(), true);
        assert_eq!(
            parser(rsx_spread_code_block).parse("{...foo}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "foo".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_spread_code_block).parse("{ ... foo }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " foo ".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_spread_code_block).parse("{...{foo}}").unwrap(),
            (
                RSXParsedExpression {
                    tokens: "{foo}".into(),
                    elements: vec![]
                },
                ""
            )
        );
        assert_eq!(
            parser(rsx_spread_code_block).parse("{ ... { foo } }").unwrap(),
            (
                RSXParsedExpression {
                    tokens: " { foo } ".into(),
                    elements: vec![]
                },
                ""
            )
        );
    }
}

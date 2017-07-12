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
use combine::combinator::{optional, parser};

use parse_elements::rsx_element;
use parse_elements_types::RSXElement;
use parse_js::js_whitespace;

pub fn rsx_element_ignoring_ws<I>(input: I) -> ParseResult<RSXElement, I>
where
    I: Stream<Item = char>
{
    optional(parser(js_whitespace))
        .with(parser(rsx_element).skip(parser(js_whitespace)))
        .parse_stream(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use parse_attributes_types::{RSXAttribute, RSXAttributeName, RSXAttributeValue, RSXAttributes};
    use parse_children_types::{RSXChild, RSXChildren, RSXText};
    use parse_elements_types::{RSXElement, RSXElementName, RSXNormalElement, RSXSelfClosingElement};
    use parse_external_placeholders::RSXElementPlaceholder;
    use parse_external_types::RSXParsedExpression;

    #[test]
    pub fn test_rsx_simple_expression() {
        assert_eq!(
            parser(rsx_element_ignoring_ws).parse(r#"<div>Hello world!</div>"#).unwrap(),
            (
                RSXElement::Normal(RSXNormalElement(
                    RSXElementName::Name("div".into()),
                    RSXAttributes::from(vec![]),
                    RSXChildren::from(vec![RSXChild::Text(RSXText("Hello world!".into()))])
                )),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_complex() {
        assert_eq!(
            parser(rsx_element_ignoring_ws)
                .parse(
                    r#"
                        <root
                            first
                            second={true}
                            third={false}
                            fourth={1}
                            fifth='2'
                            sixth="3"
                            seventh={'4'}
                            eighth={"5"}
                            ninth={6 + 7 - 8 * 9 / 10}
                            tenth={|e: Event| { println!("{:?}", e); }}
                        >
                            <div>hello</div>
                            <span>world</span>
                            {
                                if foo {
                                    <first>lorem { 1 + 2 } ipsum</first>
                                } else {
                                    <second>dolor { 3 + 4 } sit</second>
                                }
                            }
                            <ul>
                                <li>First</li>
                                <li>Second</li>
                                <li>Third</li>
                            </ul>
                            <void/>
                        </root>
                "#
                )
                .unwrap(),
            (
                RSXElement::Normal(RSXNormalElement(
                    RSXElementName::Name("root".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("first".into()),
                            RSXAttributeValue::Default
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("second".into()),
                            RSXAttributeValue::Boolean(true.into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("third".into()),
                            RSXAttributeValue::Boolean(false.into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("fourth".into()),
                            RSXAttributeValue::Number(1f64.into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("fifth".into()),
                            RSXAttributeValue::Str(("2", '"').into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("sixth".into()),
                            RSXAttributeValue::Str(("3", '\'').into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("seventh".into()),
                            RSXAttributeValue::Str(("4", '"').into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("eighth".into()),
                            RSXAttributeValue::Str(("5", '\'').into())
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("ninth".into()),
                            RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                tokens: "6 + 7 - 8 * 9 / 10".into(),
                                elements: vec![]
                            })
                        ),
                        RSXAttribute::Named(
                            RSXAttributeName::Name("tenth".into()),
                            RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                tokens: "|e: Event| { println!(\"{:?}\", e); }".into(),
                                elements: vec![]
                            })
                        ),
                    ]),
                    RSXChildren::from(vec![
                        RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name("div".into()),
                            RSXAttributes::from(vec![]),
                            RSXChildren::from(vec![RSXChild::Text(RSXText("hello".into()))])
                        ))),
                        RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name("span".into()),
                            RSXAttributes::from(vec![]),
                            RSXChildren::from(vec![RSXChild::Text(RSXText("world".into()))])
                        ))),
                        RSXChild::CodeBlock(RSXParsedExpression {
                            tokens: r#"
                                if foo {
                                    /* rsx:15848556381555908996 */
                                } else {
                                    /* rsx:13987966085338848396 */
                                }
                            "#.into(),
                            elements: vec![
                                (
                                    RSXElementPlaceholder::dummy(),
                                    RSXElement::Normal(RSXNormalElement(
                                        RSXElementName::Name("first".into()),
                                        RSXAttributes::from(vec![]),
                                        RSXChildren::from(vec![
                                            RSXChild::Text(RSXText("lorem".into())),
                                            RSXChild::CodeBlock(RSXParsedExpression {
                                                tokens: " 1 + 2 ".into(),
                                                elements: vec![]
                                            }),
                                            RSXChild::Text(RSXText("ipsum".into())),
                                        ])
                                    ))
                                ),
                                (
                                    RSXElementPlaceholder::dummy(),
                                    RSXElement::Normal(RSXNormalElement(
                                        RSXElementName::Name("second".into()),
                                        RSXAttributes::from(vec![]),
                                        RSXChildren::from(vec![
                                            RSXChild::Text(RSXText("dolor".into())),
                                            RSXChild::CodeBlock(RSXParsedExpression {
                                                tokens: " 3 + 4 ".into(),
                                                elements: vec![]
                                            }),
                                            RSXChild::Text(RSXText("sit".into())),
                                        ])
                                    ))
                                ),
                            ]
                        }),
                        RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name("ul".into()),
                            RSXAttributes::from(vec![]),
                            RSXChildren::from(vec![
                                RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                                    RSXElementName::Name("li".into()),
                                    RSXAttributes::from(vec![]),
                                    RSXChildren::from(vec![
                                        RSXChild::Text(RSXText("First".into())),
                                    ])
                                ))),
                                RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                                    RSXElementName::Name("li".into()),
                                    RSXAttributes::from(vec![]),
                                    RSXChildren::from(vec![
                                        RSXChild::Text(RSXText("Second".into())),
                                    ])
                                ))),
                                RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                                    RSXElementName::Name("li".into()),
                                    RSXAttributes::from(vec![]),
                                    RSXChildren::from(vec![
                                        RSXChild::Text(RSXText("Third".into())),
                                    ])
                                ))),
                            ])
                        ))),
                        RSXChild::Element(RSXElement::SelfClosing(RSXSelfClosingElement(
                            RSXElementName::Name("void".into()),
                            RSXAttributes::from(vec![])
                        ))),
                    ])
                )),
                ""
            )
        );
    }

    #[test]
    pub fn test_rsx_example() {
        assert_eq!(
            parser(rsx_element_ignoring_ws)
                .parse(
                    r#"
                        <Dropdown show={props.visible}>
                            A dropdown list
                            <Menu
                                icon={props.menu.icon}
                                onHide={|e| println!("{:?}", e)}
                                onShow={|e| println!("{:?}", e)}
                            >
                                <MenuItem>Do Something</MenuItem>
                                {
                                    if should_do_something_fun() {
                                        <MenuItem>Do Something Fun!</MenuItem>
                                    } else {
                                        <MenuItem>Do Something Else</MenuItem>
                                    }
                                }
                            </Menu>
                        </Dropdown>
                "#
                )
                .unwrap(),
            (
                RSXElement::Normal(RSXNormalElement(
                    RSXElementName::Name("Dropdown".into()),
                    RSXAttributes::from(vec![
                        RSXAttribute::Named(
                            RSXAttributeName::Name("show".into()),
                            RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                tokens: "props.visible".into(),
                                elements: vec![]
                            })
                        ),
                    ]),
                    RSXChildren::from(vec![
                        RSXChild::Text(RSXText("A dropdown list".into())),
                        RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name("Menu".into()),
                            RSXAttributes::from(vec![
                                RSXAttribute::Named(
                                    RSXAttributeName::Name("icon".into()),
                                    RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                        tokens: "props.menu.icon".into(),
                                        elements: vec![]
                                    })
                                ),
                                RSXAttribute::Named(
                                    RSXAttributeName::Name("onHide".into()),
                                    RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                        tokens: "|e| println!(\"{:?}\", e)".into(),
                                        elements: vec![]
                                    })
                                ),
                                RSXAttribute::Named(
                                    RSXAttributeName::Name("onShow".into()),
                                    RSXAttributeValue::CodeBlock(RSXParsedExpression {
                                        tokens: "|e| println!(\"{:?}\", e)".into(),
                                        elements: vec![]
                                    })
                                ),
                            ]),
                            RSXChildren::from(vec![
                                RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                                    RSXElementName::Name("MenuItem".into()),
                                    RSXAttributes::from(vec![]),
                                    RSXChildren::from(vec![
                                        RSXChild::Text(RSXText("Do Something".into())),
                                    ])
                                ))),
                                RSXChild::CodeBlock(RSXParsedExpression {
                                    tokens: r#"
                                    if should_do_something_fun() {
                                        /* rsx:15848556381555908996 */
                                    } else {
                                        /* rsx:13987966085338848396 */
                                    }
                                "#.into(),
                                    elements: vec![
                                        (
                                            RSXElementPlaceholder::dummy(),
                                            RSXElement::Normal(RSXNormalElement(
                                                RSXElementName::Name("MenuItem".into()),
                                                RSXAttributes::from(vec![]),
                                                RSXChildren::from(vec![
                                                    RSXChild::Text(RSXText(
                                                        "Do Something Fun!".into()
                                                    )),
                                                ])
                                            ))
                                        ),
                                        (
                                            RSXElementPlaceholder::dummy(),
                                            RSXElement::Normal(RSXNormalElement(
                                                RSXElementName::Name("MenuItem".into()),
                                                RSXAttributes::from(vec![]),
                                                RSXChildren::from(vec![
                                                    RSXChild::Text(RSXText(
                                                        "Do Something Else".into()
                                                    )),
                                                ])
                                            ))
                                        ),
                                    ]
                                }),
                            ])
                        ))),
                    ])
                )),
                ""
            )
        );
    }
}

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

use rsx_shared::types::KnownElementName;
use self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_elements_types::{
    RSXElement,
    RSXElementName,
    RSXIdentifier,
    RSXNormalElement,
    RSXSelfClosingElement
};

impl ToCustomTokens for RSXElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXElement::SelfClosing(ref element) => {
                element.to_custom_tokens(tokens);
            }
            &RSXElement::Normal(ref element) => {
                element.to_custom_tokens(tokens);
            }
        }
    }
}

impl ToCustomTokens for RSXSelfClosingElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let name = &self.0;
        let attributes = &self.1;
        let has_attributes = attributes.0.len() != 0;

        if has_attributes {
            tokens.append(quote! {
                DOMNode::from((#name, DOMAttributes::from(#attributes)))
            })
        } else {
            tokens.append(quote! {
                DOMNode::from(#name)
            })
        }
    }
}

impl ToCustomTokens for RSXNormalElement {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let name = &self.0;
        let attributes = &self.1;
        let children = &self.2;
        let has_attributes = attributes.0.len() != 0;
        let has_children = children.0.len() != 0;

        if has_attributes && has_children {
            tokens.append(quote! {
                DOMNode::from((#name, #attributes, #children))
            })
        } else if has_attributes {
            tokens.append(quote! {
                DOMNode::from((#name, DOMAttributes::from(#attributes)))
            })
        } else if has_children {
            tokens.append(quote! {
                DOMNode::from((#name, DOMChildren::from(#children)))
            })
        } else {
            tokens.append(quote! {
                DOMNode::from(#name)
            })
        }
    }
}

#[allow(unknown_lints, cyclomatic_complexity)]
impl ToCustomTokens for RSXElementName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        use self::KnownElementName::*;
        match self {
            // HTML content sectioning
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("address") => {
                RSXElementName::KnownName(Address).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("article") => {
                RSXElementName::KnownName(Article).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("aside") => {
                RSXElementName::KnownName(Aside).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("footer") => {
                RSXElementName::KnownName(Footer).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("header") => {
                RSXElementName::KnownName(Header).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("nav") => {
                RSXElementName::KnownName(Nav).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("section") => {
                RSXElementName::KnownName(Section).to_custom_tokens(tokens);
            }

            // HTML text sectioning
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("hgroup") => {
                RSXElementName::KnownName(Hgroup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h1") => {
                RSXElementName::KnownName(H1).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h2") => {
                RSXElementName::KnownName(H2).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h3") => {
                RSXElementName::KnownName(H3).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h4") => {
                RSXElementName::KnownName(H4).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h5") => {
                RSXElementName::KnownName(H5).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("h6") => {
                RSXElementName::KnownName(H6).to_custom_tokens(tokens);
            }

            // HTML text content
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("main") => {
                RSXElementName::KnownName(Main).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("div") => {
                RSXElementName::KnownName(Div).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("span") => {
                RSXElementName::KnownName(Span).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("p") => {
                RSXElementName::KnownName(P).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("ol") => {
                RSXElementName::KnownName(Ol).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("ul") => {
                RSXElementName::KnownName(Ul).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("li") => {
                RSXElementName::KnownName(Li).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dl") => {
                RSXElementName::KnownName(Dl).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dt") => {
                RSXElementName::KnownName(Dt).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("dd") => {
                RSXElementName::KnownName(Dd).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("figure") => {
                RSXElementName::KnownName(Figure).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("figcaption") => {
                RSXElementName::KnownName(Figcaption).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("hr") => {
                RSXElementName::KnownName(Hr).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("pre") => {
                RSXElementName::KnownName(Pre).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("blockquote") => {
                RSXElementName::KnownName(Blockquote).to_custom_tokens(tokens);
            }

            // HTML inline text semantics
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("a") => {
                RSXElementName::KnownName(A).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("b") => {
                RSXElementName::KnownName(Bold).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("i") => {
                RSXElementName::KnownName(Italic).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("u") => {
                RSXElementName::KnownName(Underline).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("s") => {
                RSXElementName::KnownName(Strikethrough).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("em") => {
                RSXElementName::KnownName(Emphasis).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("mark") => {
                RSXElementName::KnownName(Mark).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("q") => {
                RSXElementName::KnownName(Quotation).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("cite") => {
                RSXElementName::KnownName(Citation).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("code") => {
                RSXElementName::KnownName(Code).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("data") => {
                RSXElementName::KnownName(Data).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("time") => {
                RSXElementName::KnownName(Time).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sub") => {
                RSXElementName::KnownName(Sub).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sup") => {
                RSXElementName::KnownName(Sup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("br") => {
                RSXElementName::KnownName(Br).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("wbr") => {
                RSXElementName::KnownName(Wbr).to_custom_tokens(tokens);
            }

            // HTML media and links
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("img") => {
                RSXElementName::KnownName(Image).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("area") => {
                RSXElementName::KnownName(Area).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("map") => {
                RSXElementName::KnownName(Map).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("audio") => {
                RSXElementName::KnownName(Audio).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("video") => {
                RSXElementName::KnownName(Video).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("track") => {
                RSXElementName::KnownName(Track).to_custom_tokens(tokens);
            }

            // HTML forms
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("button") => {
                RSXElementName::KnownName(Button).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("datalist") => {
                RSXElementName::KnownName(Datalist).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("fieldset") => {
                RSXElementName::KnownName(Fieldset).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("form") => {
                RSXElementName::KnownName(Form).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("input") => {
                RSXElementName::KnownName(Input).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("label") => {
                RSXElementName::KnownName(Label).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("legend") => {
                RSXElementName::KnownName(Legend).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("meter") => {
                RSXElementName::KnownName(Meter).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("optgroup") => {
                RSXElementName::KnownName(Optgroup).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("option") => {
                RSXElementName::KnownName(Option).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("output") => {
                RSXElementName::KnownName(Output).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("progress") => {
                RSXElementName::KnownName(Progress).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("select") => {
                RSXElementName::KnownName(Select).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("textarea") => {
                RSXElementName::KnownName(Textarea).to_custom_tokens(tokens);
            }

            // React Fiber components
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("fragment") => {
                RSXElementName::KnownName(Fragment).to_custom_tokens(tokens);
            }

            // React Native basic components
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("view") => {
                RSXElementName::KnownName(View).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("text") => {
                RSXElementName::KnownName(Text).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("image") => {
                RSXElementName::KnownName(Image).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("textinput") => {
                RSXElementName::KnownName(TextInput).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("scrollview") => {
                RSXElementName::KnownName(ScrollView).to_custom_tokens(tokens);
            }

            // React Native user interface
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("picker") => {
                RSXElementName::KnownName(Picker).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("slider") => {
                RSXElementName::KnownName(Slider).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("switch") => {
                RSXElementName::KnownName(Switch).to_custom_tokens(tokens);
            }

            // React Native list views
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("flatlist") => {
                RSXElementName::KnownName(FlatList).to_custom_tokens(tokens);
            }
            &RSXElementName::Name(ref n) if n.0.eq_ignore_ascii_case("sectionlist") => {
                RSXElementName::KnownName(SectionList).to_custom_tokens(tokens);
            }

            // Parsed
            &RSXElementName::KnownName(ref n) => {
                tokens.append(quote! { DOMTagName::from(#n) });
            }
            &RSXElementName::Name(ref n) => {
                tokens.append(quote! { DOMTagName::from(#n) });
            }
            &RSXElementName::NamedspacedName(ref ns, ref n) => {
                tokens.append(quote! { DOMTagName::from((#ns, #n)) });
            }
            &RSXElementName::MemberExpression(ref member_expression) => {
                let mut inner_tokens = Tokens::new();
                member_expression.to_custom_tokens(&mut inner_tokens);
                tokens.append(quote! { DOMTagName::from(#inner_tokens) });
            }
        }
    }
}

impl ToCustomTokens for RSXIdentifier {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.0.as_ref();
        string_ref.to_custom_tokens(tokens);
    }
}

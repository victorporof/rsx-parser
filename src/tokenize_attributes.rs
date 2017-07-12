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

use rsx_shared::types::KnownAttributeName;
use self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_attributes_types::{
    RSXAttribute,
    RSXAttributeBoolean,
    RSXAttributeName,
    RSXAttributeNumber,
    RSXAttributeString,
    RSXAttributeValue,
    RSXAttributes
};

use parse_js_types::{JSDoubleStringCharacters, JSSingleStringCharacters};

impl ToCustomTokens for RSXAttributes {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let attributes = &self.0;
        tokens.append(quote! { vec!#attributes });
    }
}

impl ToCustomTokens for RSXAttribute {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttribute::Named(ref n, ref v) => {
                tokens.append(quote! { DOMAttribute::from((#n, #v)) });
            }
            &RSXAttribute::Spread(ref spread) => {
                tokens.append(quote! { DOMAttribute::from(#spread) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeName {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        use self::KnownAttributeName::*;
        match self {
            // HTML global attributes
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("accesskey") => {
                RSXAttributeName::KnownName(Accesskey).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("class") => {
                RSXAttributeName::KnownName(Class).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("contenteditable") => {
                RSXAttributeName::KnownName(CntEditable).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("contextmenu") => {
                RSXAttributeName::KnownName(Contextmenu).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("dir") => {
                RSXAttributeName::KnownName(Dir).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("draggable") => {
                RSXAttributeName::KnownName(Draggable).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("dropzone") => {
                RSXAttributeName::KnownName(Dropzone).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("hidden") => {
                RSXAttributeName::KnownName(Hidden).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("id") => {
                RSXAttributeName::KnownName(Id).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("lang") => {
                RSXAttributeName::KnownName(Lang).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("spellcheck") => {
                RSXAttributeName::KnownName(Spellcheck).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("src") => {
                RSXAttributeName::KnownName(Src).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("style") => {
                RSXAttributeName::KnownName(Style).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("tabindex") => {
                RSXAttributeName::KnownName(Tabindex).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("title") => {
                RSXAttributeName::KnownName(Title).to_custom_tokens(tokens);
            }
            &RSXAttributeName::Name(ref n) if n.0.eq_ignore_ascii_case("translate") => {
                RSXAttributeName::KnownName(Translate).to_custom_tokens(tokens);
            }

            // Parsed
            &RSXAttributeName::KnownName(ref n) => {
                tokens.append(quote! { DOMAttributeName::from(#n) });
            }
            &RSXAttributeName::Name(ref n) => {
                tokens.append(quote! { DOMAttributeName::from(#n) });
            }
            &RSXAttributeName::NamedspacedName(ref ns, ref n) => {
                tokens.append(quote! { DOMAttributeName::from((#ns, #n)) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeValue {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttributeValue::Default => {
                tokens.append(quote! { DOMAttributeValue::from(true) });
            }
            &RSXAttributeValue::Boolean(ref boolean) => {
                tokens.append(quote! { DOMAttributeValue::from(#boolean) });
            }
            &RSXAttributeValue::Number(ref number) => {
                tokens.append(quote! { DOMAttributeValue::from(#number) });
            }
            &RSXAttributeValue::Str(ref string) => {
                tokens.append(quote! { DOMAttributeValue::from(#string) });
            }
            &RSXAttributeValue::Element(ref element) => {
                tokens.append(quote! { DOMAttributeValue::from(#element) });
            }
            &RSXAttributeValue::CodeBlock(ref expression) => {
                tokens.append(quote! { DOMAttributeValue::from(#expression) });
            }
        }
    }
}

impl ToCustomTokens for RSXAttributeBoolean {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.0.to_custom_tokens(tokens);
    }
}
impl ToCustomTokens for RSXAttributeNumber {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        self.0.to_custom_tokens(tokens);
    }
}

impl ToCustomTokens for RSXAttributeString {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXAttributeString::SingleQuoted(JSSingleStringCharacters(ref chars))
            | &RSXAttributeString::DoubleQuoted(JSDoubleStringCharacters(ref chars)) => {
                let string_ref: &str = chars.as_ref();
                string_ref.to_custom_tokens(tokens);
            }
        }
    }
}

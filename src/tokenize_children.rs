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

use self_tokenize_trait::{ToCustomTokens, Tokens};

use parse_children_types::{RSXChild, RSXChildren, RSXText};

impl ToCustomTokens for RSXChildren {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let children = &self.0;
        tokens.append(quote! { vec!#children });
    }
}

impl ToCustomTokens for RSXChild {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        match self {
            &RSXChild::Element(ref element) => element.to_custom_tokens(tokens),
            &RSXChild::Text(ref text) => tokens.append(quote! { DOMNode::from(#text) }),
            &RSXChild::CodeBlock(ref code) => tokens.append(quote! { DOMNode::from(#code) })
        }
    }
}

impl ToCustomTokens for RSXText {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let string_ref: &str = self.0.as_ref();
        tokens.append(quote! { #string_ref });
    }
}

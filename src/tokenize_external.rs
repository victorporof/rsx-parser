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

use parse_external_types::RSXParsedExpression;

impl ToCustomTokens for RSXParsedExpression {
    fn to_custom_tokens(&self, tokens: &mut Tokens) {
        let mut code = format!("{}{}{}", "{", self.tokens, "}");

        self.elements.iter().for_each(|&(ref placeholder, ref element)| {
            let mut inner_tokens = Tokens::new();
            element.to_custom_tokens(&mut inner_tokens);
            code = code.replace(placeholder.as_ref(), &inner_tokens.to_string());
        });

        tokens.append(code);
    }
}

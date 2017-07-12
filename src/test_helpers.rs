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

use types::{
    JSBool,
    JSDoubleStringCharacter,
    JSDoubleStringCharacters,
    JSIdentifierPart,
    JSIdentifierStart,
    JSNumber,
    JSSingleStringCharacter,
    JSSingleStringCharacters,
    RSString,
    RSXAttribute,
    RSXAttributeBoolean,
    RSXAttributeName,
    RSXAttributeNumber,
    RSXAttributeString,
    RSXAttributeValue,
    RSXAttributes,
    RSXChild,
    RSXChildren,
    RSXElementName,
    RSXIdentifier,
    RSXNormalElement,
    RSXOpeningElement,
    RSXSelfClosingElement,
    RSXText,
    RSXTextCharacter
};

// JS Types

impl From<bool> for JSBool {
    fn from(v: bool) -> Self {
        JSBool(v)
    }
}

impl From<f64> for JSNumber {
    fn from(v: f64) -> Self {
        JSNumber(v)
    }
}

impl From<char> for JSSingleStringCharacter {
    fn from(v: char) -> Self {
        JSSingleStringCharacter(v)
    }
}

impl From<&'static str> for JSSingleStringCharacters {
    fn from(v: &'static str) -> Self {
        JSSingleStringCharacters(v.into())
    }
}

impl From<char> for JSDoubleStringCharacter {
    fn from(v: char) -> Self {
        JSDoubleStringCharacter(v)
    }
}

impl From<&'static str> for JSDoubleStringCharacters {
    fn from(v: &'static str) -> Self {
        JSDoubleStringCharacters(v.into())
    }
}

impl From<char> for JSIdentifierStart {
    fn from(v: char) -> Self {
        JSIdentifierStart(v)
    }
}

impl From<&'static str> for JSIdentifierPart {
    fn from(v: &'static str) -> Self {
        JSIdentifierPart(v.into())
    }
}

// Rust types

impl From<&'static str> for RSString {
    fn from(v: &'static str) -> Self {
        RSString(v.into())
    }
}

// RSX Elements

impl From<&'static str> for RSXSelfClosingElement {
    fn from(n: &'static str) -> Self {
        RSXSelfClosingElement(n.into(), RSXAttributes::from(vec![]))
    }
}

impl From<(&'static str, &'static str)> for RSXSelfClosingElement {
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        RSXSelfClosingElement((ns, n).into(), RSXAttributes::from(vec![]))
    }
}

impl<'a> From<&'a [&'static str]> for RSXSelfClosingElement {
    fn from(xs: &'a [&'static str]) -> Self {
        RSXSelfClosingElement(xs.into(), RSXAttributes::from(vec![]))
    }
}

impl From<&'static str> for RSXNormalElement {
    fn from(n: &'static str) -> Self {
        RSXNormalElement(n.into(), RSXAttributes::from(vec![]), RSXChildren::from(vec![]))
    }
}

impl From<(&'static str, &'static str)> for RSXNormalElement {
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        RSXNormalElement((ns, n).into(), RSXAttributes::from(vec![]), RSXChildren::from(vec![]))
    }
}

impl<'a> From<&'a [&'static str]> for RSXNormalElement {
    fn from(xs: &'a [&'static str]) -> Self {
        RSXNormalElement(xs.into(), RSXAttributes::from(vec![]), RSXChildren::from(vec![]))
    }
}

impl From<&'static str> for RSXOpeningElement {
    fn from(n: &'static str) -> Self {
        RSXOpeningElement(n.into(), RSXAttributes::from(vec![]))
    }
}

impl From<(&'static str, &'static str)> for RSXOpeningElement {
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        RSXOpeningElement((ns, n).into(), RSXAttributes::from(vec![]))
    }
}

impl<'a> From<&'a [&'static str]> for RSXOpeningElement {
    fn from(xs: &'a [&'static str]) -> Self {
        RSXOpeningElement(xs.into(), RSXAttributes::from(vec![]))
    }
}

impl From<&'static str> for RSXElementName {
    fn from(n: &'static str) -> Self {
        RSXElementName::Name(n.into())
    }
}

impl From<(&'static str, &'static str)> for RSXElementName {
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        RSXElementName::NamedspacedName(ns.into(), n.into())
    }
}

impl<'a> From<&'a [&'static str]> for RSXElementName {
    fn from(xs: &'a [&'static str]) -> Self {
        let vec = xs.into_iter().map(|v| (*v).into()).collect::<Vec<_>>();
        RSXElementName::MemberExpression(vec.into())
    }
}

impl From<&'static str> for RSXIdentifier {
    fn from(v: &'static str) -> Self {
        RSXIdentifier(v.into())
    }
}

// RSX Attributes

impl From<(&'static str, &'static str)> for RSXAttribute {
    fn from((k, v): (&'static str, &'static str)) -> Self {
        RSXAttribute::Named(k.into(), v.into())
    }
}

impl From<((&'static str, &'static str), &'static str)> for RSXAttribute {
    fn from(((kns, kn), v): ((&'static str, &'static str), &'static str)) -> Self {
        RSXAttribute::Named((kns, kn).into(), v.into())
    }
}

impl From<&'static str> for RSXAttributeName {
    fn from(n: &'static str) -> Self {
        RSXAttributeName::Name(n.into())
    }
}

impl From<(&'static str, &'static str)> for RSXAttributeName {
    fn from((ns, n): (&'static str, &'static str)) -> Self {
        RSXAttributeName::NamedspacedName(ns.into(), n.into())
    }
}

impl From<bool> for RSXAttributeValue {
    fn from(v: bool) -> Self {
        RSXAttributeValue::Boolean(v.into())
    }
}

impl From<f64> for RSXAttributeValue {
    fn from(v: f64) -> Self {
        RSXAttributeValue::Number(v.into())
    }
}

impl From<&'static str> for RSXAttributeValue {
    fn from(v: &'static str) -> Self {
        if v == "true" {
            RSXAttributeValue::Default
        } else {
            RSXAttributeValue::Str(v.into())
        }
    }
}

impl From<(&'static str, char)> for RSXAttributeValue {
    fn from((v, t): (&'static str, char)) -> Self {
        if v == "true" {
            RSXAttributeValue::Default
        } else {
            RSXAttributeValue::Str((v, t).into())
        }
    }
}

impl From<bool> for RSXAttributeBoolean {
    fn from(v: bool) -> Self {
        RSXAttributeBoolean(v)
    }
}

impl From<f64> for RSXAttributeNumber {
    fn from(v: f64) -> Self {
        RSXAttributeNumber(v)
    }
}

impl From<&'static str> for RSXAttributeString {
    fn from(v: &'static str) -> Self {
        RSXAttributeString::SingleQuoted(v.into())
    }
}

impl From<(&'static str, char)> for RSXAttributeString {
    fn from((n, t): (&'static str, char)) -> Self {
        match t {
            '"' => RSXAttributeString::SingleQuoted(n.into()),
            '\'' => RSXAttributeString::DoubleQuoted(n.into()),
            _ => panic!("Unsupported string format")
        }
    }
}

// RSX Children

impl From<&'static str> for RSXChild {
    fn from(v: &'static str) -> Self {
        RSXChild::Text(v.into())
    }
}

impl From<&'static str> for RSXText {
    fn from(v: &'static str) -> Self {
        RSXText(v.into())
    }
}

impl From<char> for RSXTextCharacter {
    fn from(v: char) -> Self {
        RSXTextCharacter(v)
    }
}

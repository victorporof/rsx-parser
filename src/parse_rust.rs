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
use combine::combinator::{between, many, none_of, parser, token, try};

use parse_js::{js_comment, js_whitespace};
use parse_misc::escaped_character;
use parse_rust_types::{RSChar, RSString};

pub fn rs_char<I>(input: I) -> ParseResult<RSChar, I>
where
    I: Stream<Item = char>
{
    between(token('\''), token('\''), choice!(try(parser(escaped_character)), none_of("'".chars())))
        .map(RSChar)
        .parse_stream(input)
}

pub fn rs_string<I>(input: I) -> ParseResult<RSString, I>
where
    I: Stream<Item = char>
{
    between(
        token('"'),
        token('"'),
        many(choice!(try(parser(escaped_character)), none_of("\"".chars())))
    ).map(RSString)
        .parse_stream(input)
}

pub fn rs_comment<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    parser(js_comment).parse_stream(input)
}

pub fn rs_whitespace<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    parser(js_whitespace).parse_stream(input)
}

**Under heavy research and development, please don't use this yet!**

# rsx-parser
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)
[![Build Status](https://travis-ci.org/victorporof/rsx-parser.svg?branch=master)](https://travis-ci.org/victorporof/rsx-parser)

JSX-like parser combinator for Rust

## Purpose
This is an experimental parser for JSX-like code in Rust. The long term goal might be to build "something like React" in Rust, but this can mean a number of things, from a direct port with 100% API compatibility to a completely different product. A JSX-like parser is a good and simple place to start experimenting from.

## How to use
[Documentation](https://victorporof.github.io/rsx-parser)

This crate concerns itself strictly with parsing the RSX syntax. If you're just looking to write RSX in your project, take a look at the [RSX compiler plugin](https://github.com/victorporof/rsx_compiler_plugin) instead.

Otherwise, add this to your `Cargo.toml` file:

```toml
[dependencies]
rsx-parser = { git = "https://github.com/victorporof/rsx-parser.git" }
```

Then, simply import the library into your code to parse RSX. The parser generates an `rsx_parser::RSXElement` abstract syntax tree representing the RSX code, returning it together with the remaining unparsed input (if any). Other parser combinators can thus use this output.

```rust
extern crate rsx_parser;
use rsx_parser::parse;
use rsx_parser::types::RSXElement;

let source = "<div>Hello world!</div>";
let (ast, remaining): (RSXElement, _) = parse(source).unwrap();
```

All data structures (including the AST) are [self tokenizing](https://github.com/victorporof/rust_self_tokenize.git) values, meaning that they can serialize themselves to generate a `quote::Tokens` which can be then directly translated into a `proc_macro::TokenStream`, used for creating Rust compiler plugins as [procedural macros](https://doc.rust-lang.org/book/first-edition/procedural-macros.html). See the [syn](https://github.com/dtolnay/syn) and [quote](https://github.com/dtolnay/quote) crates for more information.

Note that procedural macros are not fully standardized as of September 2017, but sufficient features are available in the current Rust nightly version (1.22). See the [RFC](https://github.com/rust-lang/rfcs/blob/master/text/1566-proc-macros.md) and the [tracking issue](https://github.com/rust-lang/rust/issues/38356) for more information.

Use the [rust-self-tokenize](https://github.com/victorporof/rust_self_tokenize.git) or [quote](https://github.com/dtolnay/quote) crates for the `quote::Tokens` type.

```rust
extern crate self_tokenize_trait;
use self_tokenize_trait::{ToCustomTokens, Tokens};

let source = "<div>{ external_rust_code() }</div>";
let (ast, remaining) = parse(source).unwrap();
let mut tokens = Tokens::new();
ast.to_custom_tokens(&mut tokens);
```

This library should work on the stable Rust channel, but if you want to use the [RSX compiler plugin](https://github.com/victorporof/rsx_compiler_plugin), then you need Nightly:
```sh
rustup default nightly
```

## RSX vs. JSX
The [JSX spec](http://facebook.github.io/jsx) is, although a draft, presumably stable. Syntax extension equivalents can be found for Rust, which is the main scope of this experiment.

Example, inspired from the JSX spec website linked above:

```jsx
const FunDropdown = (props) =>
  <Dropdown show={props.visible}>
    A dropdown list
    <Menu
      icon={props.menu.icon}
      onHide={(e) => console.log(e)}
      onShow={(e) => console.log(e)}
    >
      <MenuItem>Do Something</MenuItem>
      {
        shouldDoSomethingFun()
          ? <MenuItem>Do Something Fun!</MenuItem>
          : <MenuItem>Do Something Else</MenuItem>
      }
    </Menu>
  </Dropdown>;
```

An equivalent interpretation of JSX in Rust, using compiler plugins, looks this:

```rust
fn fun_dropdown(props: Props) -> RSXElement {
  rsx! {
    <Dropdown show={props.visible}>
      A dropdown list
      <Menu
        icon={props.menu.icon}
        onHide={|e: Event| println!("{:?}", e)}
        onShow={|e: Event| println!("{:?}", e)}
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
  }
}
```

## Supported grammar
All of the [JSX official grammar](http://facebook.github.io/jsx) is supported. In the case of handling arbitrary Rust code inside RSX, the treatment is similar: JSX can contain JavaScript "code blocks" delimited by curly braces (specifically "assignment expressions"), in clearly defined locations such as attribute values, children contents etc. Rust expressions provide sufficient equivalence.

### PrimaryExpression
- [X] JSXElement

### Elements

#### JSXElement
- [X] JSXSelfClosingElement
- [X] JSXOpeningElement JSXChildren? JSXClosingElement

#### JSXSelfClosingElement
- [X] `<` JSXElementName JSXAttributes? `/` `>`

#### JSXOpeningElement
- [X] `<` JSXElementName JSXAttributes? `>`

#### JSXClosingElement
- [X] `<` `/` JSXElementName `>`

#### JSXElementName
- [X] JSXIdentifier
- [X] JSXNamedspacedName
- [X] JSXMemberExpression

#### JSXIdentifier
- [X] IdentifierStart
- [X] JSXIdentifier IdentifierPart
- [X] JSXIdentifier `-`

#### JSXNamespacedName
- [X] JSXIdentifier `:` JSXIdentifier

#### JSXMemberExpression
- [X] JSXIdentifier `.` JSXIdentifier
- [X] JSXMemberExpression `.` JSXIdentifier

### Attributes

#### JSXAttributes
- [X] JSXSpreadAttribute JSXAttributes?
- [X] JSXAttribute JSXAttributes?

#### JSXSpreadAttribute
- [X] `{` ... AssignmentExpression `}`

#### JSXAttribute
- [X] JSXAttributeName `=` JSXAttributeValue

#### JSXAttributeName
- [X] JSXIdentifier
- [X] JSXNamespacedName

#### JSXAttributeValue
- [X] `"` JSXDoubleStringCharacters? `"`
- [X] `'` JSXSingleStringCharacters? `'`
- [X] `{` AssignmentExpression `}`
- [X] JSXElement

#### JSXDoubleStringCharacters
- [X] JSXDoubleStringCharacter JSXDoubleStringCharacters?

#### JSXDoubleStringCharacter
- [X] SourceCharacter *but not* `"`

#### JSXSingleStringCharacters
- [X] JSXSingleStringCharacter JSXSingleStringCharacters?

#### JSXSingleStringCharacter
- [X] SourceCharacter *but not* `'`

### Children

#### JSXChildren
- [X] JSXChild JSXChildren?

#### JSXChild
- [X] JSXText
- [X] JSXElement
- [X] `{` AssignmentExpression? `}`

#### JSXText
- [X] JSXTextCharacter JSXText?

#### JSXTextCharacter
- [X] SourceCharacter *but not one of* `{`, `<`, `>` *or* `}`

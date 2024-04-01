# `demangle`

[![Rust](https://github.com/valentinegb/demangle/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/demangle/actions/workflows/rust.yml)

CLI tool for quickly demangling a symbol

Supports the same languages as [symbolic-demangle](https://github.com/getsentry/symbolic/tree/master/symbolic-demangle): C++, Rust, Swift, and ObjC.

Powered by [symbolic](https://github.com/getsentry/symbolic) and [clap](https://github.com/clap-rs/clap).

## Usage

```txt
demangle <LANGUAGE> <SYMBOL>
```

Use `demangle --help` for more.

## Example

```txt
% demangle swift '_$s10Foundation15AttributeScopesO7SwiftUIE05swiftE0AcDE0D12UIAttributesVmvg'
(extension in SwiftUI):Foundation.AttributeScopes.swiftUI.getter : (extension in SwiftUI):Foundation.AttributeScopes.SwiftUIAttributes.Type
```

## Installation

```sh
cargo install demangle
```

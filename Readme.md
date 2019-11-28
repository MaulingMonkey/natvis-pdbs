# natvis-pdbs

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/natvis-pdbs.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/natvis-pdbs)
<!--[![Build status](https://ci.appveyor.com/api/projects/status/nyvlrelifcyjc1l1?svg=true)](https://ci.appveyor.com/project/MaulingMonkey/natvis-pdbs)-->
[![Crates.io](https://img.shields.io/crates/v/natvis-pdbs.svg)](https://crates.io/crates/natvis-pdbs)
![unsafe: no](https://img.shields.io/badge/unsafe-no-green.svg)
![rust: 1.39.0+](https://img.shields.io/badge/rust-1.39.0%2B-green.svg)
[![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/natvis-pdbs.svg)](https://github.com/MaulingMonkey/natvis-pdbs/issues)
[![License](https://img.shields.io/crates/l/natvis-pdbs.svg)](https://github.com/MaulingMonkey/natvis-pdbs)
[![Docs](https://docs.rs/natvis-pdbs/badge.svg)](https://docs.rs/natvis-pdbs/)
[![dependency status](https://deps.rs/repo/github/MaulingMonkey/natvis-pdbs/status.svg)](https://deps.rs/repo/github/MaulingMonkey/natvis-pdbs)

A [metabuild]/[build.rs] compatible crate to embed `.natvis` debug visualizer files into your executable's `.pdb`s, for ease of debugging.



## Limitations

* Natvis and PDBs being Microsoft technologies, this won't have any effect unless using MSVC-based build toolchains.
* While `crate-type=rlib` crates can contain .natvis files, they should not directly rely on `natvis-pdbs`.
  (Due to the way crates are isolated for build, `natvis-pdbs` will only work when used for the final .exe, .lib,
  or .dll crate at this time.)
* This abuses the `%LINK%` environment variable since there's currently no stable build.rs-friendly arbitrary link-args.


## Quick Start

#### To author crates containing .natvis files \([example](https://github.com/MaulingMonkey/natvis-pdbs/tree/master/crates/example-crate-with-natvis/)\)

Place a natvis files inside the root of your crate, or inside the `src` folder.
Subdirectories will *not* (currently) be searched.
See [Recommended Reading](#recommended-reading) bellow for more information about the .natvis format.

```xml
<?xml version="1.0" encoding="utf-8"?>
<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">
  <Type Name="example_crate_with_natvis::Flags">
      <Expand>
        <Item Name="bits"   ExcludeView="sparse">bits,bb</Item>
        <Item Name="A (1)"  ExcludeView="sparse">(bits &amp; (1 &lt;&lt; 0)) != 0</Item>
        <Item Name="B (2)"  ExcludeView="sparse">(bits &amp; (1 &lt;&lt; 1)) != 0</Item>
        <Item Name="C (4)"  ExcludeView="sparse">(bits &amp; (1 &lt;&lt; 2)) != 0</Item>
      </Expand>
  </Type>
</AutoVisualizer>
```

#### To consume .natvis files via [build.rs] \([example](https://github.com/MaulingMonkey/natvis-pdbs/tree/master/crates/example-usage-build-rs/)\)

Add the following to your executable's Cargo.toml:
```toml
[build-dependencies]
natvis-pdbs = "0"
```

And the following to your [build.rs]:
```rust
fn main() {
    natvis_pdbs::metabuild();
}
```

#### To consume .natvis files via [metabuild] \(nightly only, [example](https://github.com/MaulingMonkey/natvis-pdbs/tree/master/crates/example-usage-metabuild/)\)

Add the following to your executable's Cargo.toml:
```toml
cargo-features = ["metabuild"]

[package]
metabuild = ["natvis-pdbs"]

[build-dependencies]
natvis-pdbs = "0"
```



## Recommended Reading

* [Create custom views of C++ objects](https://docs.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects?view=vs-2019)
* [Format specifiers for C++ in the Visual Studio debugger](https://docs.microsoft.com/en-us/visualstudio/debugger/format-specifiers-in-cpp?view=vs-2019)
* [Context Operator in the Visual Studio Debugger (C++)](https://docs.microsoft.com/en-us/visualstudio/debugger/context-operator-cpp?view=vs-2019)



## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->

[metabuild]:                        https://github.com/rust-lang/rfcs/blob/master/text/2196-metabuild.md
[build.rs]:                         https://doc.rust-lang.org/cargo/reference/build-scripts.html

# roxmltree
[![Build Status](https://travis-ci.org/RazrFalcon/roxmltree.svg?branch=master)](https://travis-ci.org/RazrFalcon/roxmltree)
[![Crates.io](https://img.shields.io/crates/v/roxmltree.svg)](https://crates.io/crates/roxmltree)
[![Documentation](https://docs.rs/roxmltree/badge.svg)](https://docs.rs/roxmltree)

Represents an [XML 1.0](https://www.w3.org/TR/xml/) document as a read-only tree.

## Why read-only?

Because in some cases all you need is to retrieve some data from an XML document.
And for such cases, we can make a lot of optimizations.

As for *roxmltree*, it's fast not only because it's read-only, but also because
it uses [xmlparser], which is many times faster than [xml-rs].
See the [Performance](#performance) section for details.

## Parsing behavior

Sadly, XML can be parsed in many different ways. *roxmltree* tries to mimic the
behavior of Python's [lxml](https://lxml.de/).
But unlike *lxml*, *roxmltree* does support comments outside the root element.

Fo more details see [docs/parsing.md](https://github.com/RazrFalcon/roxmltree/blob/master/docs/parsing.md).

## Alternatives

| Feature/Crate                   | roxmltree        | [libxml2]        | [xmltree]        | [elementtree]    | [sxd-document]   | [treexml]        |
| ------------------------------- | :--------------: | :--------------: | :--------------: | :--------------: | :--------------: | :--------------: |
| Element namespace resolving     | ✔                | ✔                | ✔                | ✔               | ~<sup>1</sup>     |                  |
| Attribute namespace resolving   | ✔                | ✔                |                  |                  | ✔                |                  |
| [Entity references]             | ✔                | ✔                | ⚠                | ⚠                | ⚠             | ⚠                |
| [Character references]          | ✔                | ✔                | ✔                | ✔                | ✔                | ✔                |
| [Attribute-Value normalization] | ✔                | ✔                |                  |                  |                  |                  |
| Comments                        | ✔                | ✔                |                  |                  | ✔                |                  |
| Processing instructions         | ✔                | ✔                | ⚠                |                  | ✔               |                  |
| UTF-8 BOM                       | ✔                | ✔                | ⚠               | ⚠               | ⚠               | ⚠                |
| Non UTF-8 input                 |                  | ✔                 |                  |                  |                  |                  |
| Complete DTD support            |                  | ✔                |                  |                  |                  |                  |
| Position preserving<sup>2</sup> | ✔                | ✔                |                 |                 |                 |                  |
| HTML support                    |                  | ✔                |                  |                  |                  |                  |
| Tree modification               |                  | ✔                | ✔                | ✔                | ✔                | ✔                |
| Writing                         |                  | ✔                | ✔                | ✔                | ✔                | ✔                |
| No **unsafe**                   | ✔                |                  | ✔                | ~<sup>3</sup>    |                  | ✔                |
| Language                        | Rust             | C                | Rust             | Rust             | Rust             | Rust             |
| Size overhead<sup>4</sup>       | **~62KiB**       | ~1.4MiB          | ~80KiB           | ~96KiB           | ~130KiB          | ~110KiB          |
| Dependencies                    | **1**            | ?<sup>5</sup>    | 2                | 18               | 2                | 14               |
| Tested version                  | 0.4.1            | 2.9.8            | 0.8.0            | 0.5.0            | 0.3.0            | 0.7.0            |
| License                         | MIT / Apache-2.0 | MIT              | MIT              | BSD-3-Clause     | MIT              | MIT              |

Legend:

- ✔ - supported
- ⚠ - parsing error
- ~ - partial
- *nothing* - not supported

Notes:

1. No default namespace propagation.
2. *roxmltree* keeps all node and attribute positions in the original document,
   so you can easily retrieve it if you need it.
   See [examples/print_pos.rs](examples/print_pos.rs) for details.
3. In the `string_cache` crate.
4. Binary size overhead according to [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat).
5. Depends on build flags.

[Entity references]: https://www.w3.org/TR/REC-xml/#dt-entref
[Character references]: https://www.w3.org/TR/REC-xml/#NT-CharRef
[Attribute-Value Normalization]: https://www.w3.org/TR/REC-xml/#AVNormalize

[libxml2]: http://xmlsoft.org/
[xmltree]: https://crates.io/crates/xmltree
[elementtree]: https://crates.io/crates/elementtree
[treexml]: https://crates.io/crates/treexml
[sxd-document]: https://crates.io/crates/sxd-document

## Performance

```text
test large_roxmltree     ... bench:   5,660,398 ns/iter (+/- 104,723)
test large_sdx_document  ... bench:   9,562,290 ns/iter (+/- 45,585)
test large_xmltree       ... bench:  32,797,800 ns/iter (+/- 134,016)
test large_treexml       ... bench:  31,380,063 ns/iter (+/- 71,732)
test large_elementtree   ... bench:  32,121,979 ns/iter (+/- 264,842)

test medium_roxmltree    ... bench:   1,094,562 ns/iter (+/- 6,030)
test medium_sdx_document ... bench:   3,791,252 ns/iter (+/- 41,929)
test medium_treexml      ... bench:  10,975,247 ns/iter (+/- 22,692)
test medium_xmltree      ... bench:  11,601,320 ns/iter (+/- 46,216)
test medium_elementtree  ... bench:  11,550,227 ns/iter (+/- 17,991)

test tiny_roxmltree      ... bench:       7,697 ns/iter (+/- 41)
test tiny_sdx_document   ... bench:      28,031 ns/iter (+/- 101)
test tiny_xmltree        ... bench:      47,199 ns/iter (+/- 110)
test tiny_treexml        ... bench:      50,399 ns/iter (+/- 55)
test tiny_elementtree    ... bench:      51,569 ns/iter (+/- 165)
```

*roxmltree* uses [xmlparser] internally,
while *sdx-document* uses its own implementation and *xmltree*, *elementtree*
and *treexml* use the [xml-rs] crate.
Here is a comparison between *xmlparser* and *xml-rs*:

```text
test large_xmlparser     ... bench:   2,485,710 ns/iter (+/- 1,246)
test large_xmlrs         ... bench:  28,252,304 ns/iter (+/- 27,852)

test medium_xmlparser    ... bench:     624,192 ns/iter (+/- 1,040)
test medium_xmlrs        ... bench:  10,237,568 ns/iter (+/- 13,497)

test tiny_xmlparser      ... bench:       4,986 ns/iter (+/- 84)
test tiny_xmlrs          ... bench:      45,832 ns/iter (+/- 50)
```

You can try it yourself by running `cargo bench` in the `benches` dir.

Notes:

- Since all libraries have a different XML support, benchmarking is a bit pointless.
- Tree crates may use different *xml-rs* crate versions.
- We do not bench the libxml2, because `xmlReadFile()` will parse only an XML structure,
  without attributes normalization and stuff. So it's hard to compare.
  And we have to use a separate benchmark utility.

[xml-rs]: https://crates.io/crates/xml-rs
[xmlparser]: https://crates.io/crates/xmlparser

## Safety

- This library must not panic. Any panic should be considered a critical bug and reported.
- This library forbids `unsafe` code.

## Non-goals

- Complete XML support.
- Tree modification and writing.
- XPath/XQuery.

## API

This library uses Rust's idiomatic API based on iterators.
In case you are more familiar with browser/JS DOM APIs - you can check out
[tests/dom-api.rs](tests/dom-api.rs) to see how it can be converted into a Rust one.

## Dependency

[Rust](https://www.rust-lang.org/) >= 1.18

## License

Licensed under either of

- [Apache License v2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

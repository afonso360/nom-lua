# nom-lua [![crates-badge][]][crates] [![travis-badge][]][travis] [![appveyor-badge][]][appveyor] [![coveralls-badge][]][coveralls] [![license-badge][]][license] [![docs-badge][]][docs]

Lua 5.3 parser written with nom

# Syntax
- [ ] chunk (needs tests)
- [x] block
- [ ] stat
  - [x] ";"
  - [ ] varlist = explist
  - [ ] functioncall (deps functioncall)
  - [x] label
  - [x] "break"
  - [x] goto
  - [ ] do end
  - [ ] while
  - [ ] repeat until
  - [ ] if
  - [ ] for
  - [ ] for in
  - [ ] function (deps funcname)
  - [ ] local function
  - [ ] local
- [x] retstat  (needs tests)
- [x] label
- [ ] varlist (deps var)
- [x] var
- [x] namelist (needs tests)
- [x] explist (needs tests)
- [x] exp (needs tests)
  - [x] Numeral
  - [x] Nil
  - [x] Bool
  - [x] LiteralString
  - [x] "..."
  - [x] functiondef
  - [x] prefixexp
  - [x] tableconstructor
  - [x] op
- [ ] prefixexp (needs to be finished)
- [ ] functioncall (deps args, prefixexp)
- [ ] args (deps explist, tableconstructor, LiteralString)
- [x] funcname
- [x] functiondef
- [x] funcbody
- [x] parlist
- [x] tableconstructor (needs tests)
- [x] fieldlist (needs tests)
- [x] field
- [x] fieldsep
- [x] Binop (percedence error)
- [x] Unop
- [x] Name
- [x] Numeral
  - [X] Digit
  - [X] Hex Digit
  - [X] Float
  - [ ] Hex Float
- [x] LitrealString
  - [ ] Short Literal
    - [x] linebreaks
    - [ ] byte
    - [x] unicode
    - [x] escape
    - [ ] '\z'
  - [ ] Literal


# TODO
- [ ] Different integer and floating point values (i32, f32)
- [ ] Better syntax errors
- [x] Fallback to floats on overflow
- [ ] Benchmarks
- [ ] Fuzzing
- [ ] Change tests to fail instead of panicking
- [ ] Enforce ASTNodes correctness on more operations
	For example, BinOp could take a BinOp enum instead of ASTNode
- [ ] Build all features on CI


## License

nom-lua is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[crates-badge]: https://img.shields.io/crates/v/nom-lua.svg?style=flat-square
[travis-badge]: https://img.shields.io/travis/afonso360/nom-lua/master.svg?style=flat-square
[appveyor-badge]: https://img.shields.io/appveyor/ci/afonso360/nom-lua/master.svg?style=flat-square
[coveralls-badge]: https://img.shields.io/coveralls/afonso360/nom-lua/master.svg?style=flat-square
[license-badge]: https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue.svg?style=flat-square
[docs-badge]: https://img.shields.io/badge/docs-0.0.2-blue.svg?style=flat-square
[travis]: https://travis-ci.org/afonso360/nom-lua
[appveyor]: https://ci.appveyor.com/project/afonso360/nom-lua
[coveralls]: https://coveralls.io/github/afonso360/nom-lua
[docs]: https://docs.rs/nom-lua/
[license]: https://github.com/afonso360/nom-lua#license
[crates]: https://crates.io/crates/nom-lua

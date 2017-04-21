# nom-lua [![travis-badge][]][travis] [![appveyor-badge][]][appveyor] [![coveralls-badge][]][coveralls] [![license-badge][]][license] [![docs-badge][]][docs]

Lua 5.3 parser written with nom

# Syntax
- [ ] chunk (deps block)
- [ ] block (deps stat, restat)
- [ ] stat (deps everything)
  - [x] ";"
  - [ ] varlist = explist (deps explis)
  - [ ] functioncall (deps functioncall)
  - [x] label
  - [x] "break"
  - [x] goto
  - [ ] do end (deps block)
  - [ ] while (deps exp, block)
  - [ ] repeat until (deps exp, block)
  - [ ] if (deps exp, block)
  - [ ] for (deps exp, block)
  - [ ] for in (deps exp, explist, block)
  - [ ] function (deps funcname)
  - [ ] local function
  - [ ] local (deps namelist, explist)
- [ ] retstat
- [x] label
- [ ] funcname
- [ ] varlist (deps var)
- [ ] var (deps prefixexp, exp)
- [x] namelist
- [ ] explist (deps exp)
- [ ] exp
  - [x] Numeral
  - [x] Nil
  - [x] Bool
  - [x] LiteralString
  - [x] "..."
  - [ ] functiondef - can be done
  - [ ] prefixexp
  - [ ] tableconstructor
  - [ ] op (recurse bug)
- [ ] prefixexp (needs to be finished)
- [ ] functioncall (deps args, prefixexp)
- [ ] args (deps explist, tableconstructor, LiteralString)
- [ ] functiondef (deps funcbody)
- [ ] funcbody (deps parlist, block)
- [ ] parlist
- [ ] tableconstructor (deps fieldlist)
- [ ] fieldlist (deps field)
- [ ] field (deps, exp)
- [x] fieldsep
- [x] Binop (percedence error)
- [x] Unop
- [x] Name
- [x] Numeral
  - [X] Digit (does not overflow correctly)
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
- [ ] Fallback to floats on overflow
- [ ] Benchmarks
- [ ] Fuzzing
- [ ] Change tests to fail instead of panicking


## License

nom-lua is licensed under GPL General Public License 2.0 only with a Linking exception

This means that you can link it with your program even if its license is not GPL

Read [LICENSE][license] for more information.

[travis-badge]: https://img.shields.io/travis/afonso360/nom-lua/master.svg?style=flat-square
[appveyor-badge]: https://img.shields.io/appveyor/ci/afonso360/nom-lua/master.svg?style=flat-square
[coveralls-badge]: https://img.shields.io/coveralls/afonso360/nom-lua/master.svg?style=flat-square
[license-badge]: https://img.shields.io/badge/license-GPLv2%20With%20Linking%20exception-blue.svg?style=flat-square
[docs-badge]: https://img.shields.io/badge/docs-0.0.1-blue.svg?style=flat-square
[travis]: https://travis-ci.org/afonso360/nom-lua
[appveyor]: https://ci.appveyor.com/project/afonso360/nom-lua
[coveralls]: https://coveralls.io/github/afonso360/nom-lua
[docs]: https://docs.rs/nom-lua/0.0.1/nom-lua/
[license]: LICENSE

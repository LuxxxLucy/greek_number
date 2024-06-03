Greek Numeber
====================

[![CI](https://github.com/luxxxlucy/greek_number/actions/workflows/ci.yml/badge.svg)](https://github.com/luxxxlucy/greek_number/actions/workflows/ci.yml)

Convert numbers to Greek numbers

It implements the single digit M power representation from
[The Greek Number Converter](https://www.russellcottrell.com/greek/utilities/GreekNumberConverter.htm) and also
described in [Greek Numbers](https://mathshistory.st-andrews.ac.uk/HistTopics/Greek_numbers/),
it can represent numbers up to 9,999,999,999,999,999,999,999,999,999,999,999,999,999.

## Example

```
use greek_number::{to_greek_lowercase, to_greek_uppercase};

println!("{}", to_greek_lowercase(1)); // "αʹ"
println!("{}", to_greek_lowercase(241)); // "σμαʹ"
println!("{}", to_greek_uppercase(1)); // "Αʹ"
println!("{}", to_greek_uppercase(241)); // "ΣΜΑʹ"

println!("{}", to_greek_uppercase(97_554)); //  "αΜθʹ, ͵ζφνδ"
println!("{}", to_greek_uppercase(2_056_839_184)); // "βΜκʹ, αΜ͵εχπγ, ͵θρπδ"
println!("{}", to_greek_uppercase(12_312_398_676)); // "βΜρκγʹ, αΜ͵ασλθ, ͵ηχοϛ"
println!("{}", to_greek_uppercase(2_000_000_000)); "βΜκʹ"
println!("{}", to_greek_uppercase(90_000_001)); "αΜ͵θ, αʹ"

```


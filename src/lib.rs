pub enum Case {
    /// Everything is lowercased.
    Lower,
    /// Everything is uppercased.
    Upper,
}

/// Stringify a number to Greek numbers in lowercase
///
/// Greek numbers use the Greek Alphabet to represent numbers; it is based on 10 (decimal).
/// Here we implement the single digit M power representation from
/// [The Greek Number Converter](https://www.russellcottrell.com/greek/utilities/GreekNumberConverter.htm) and also
/// described in [Greek Numbers](https://mathshistory.st-andrews.ac.uk/HistTopics/Greek_numbers/)
///
/// # Examples
///
/// ```
/// use greek_number::to_greek_lowercase;
///
/// let greek = to_greek_lowercase(1);
/// println!("{}", greek); // "αʹ"
///
/// let greek = to_greek_lowercase(241);
/// println!("{}", greek); // "σμαʹ"
/// ```
pub fn to_greek_lowercase(n: usize) -> String {
    to_greek(n, Case::Lower)
}

/// Stringify a number to Greek numbers in uppercase
///
/// # Examples
///
/// ```
/// use greek_number::to_greek_uppercase;
///
/// let greek = to_greek_uppercase(1);
/// println!("{}", greek); // "Αʹ"
///
/// let greek = to_greek_uppercase(241);
/// println!("{}", greek); // "ΣΜΑʹ"
/// ```
pub fn to_greek_uppercase(n: usize) -> String {
    to_greek(n, Case::Upper)
}

#[allow(non_snake_case)]
fn to_greek(n: usize, case: Case) -> String {
    if n == 0 {
        return '𐆊'.into(); // Greek Zero Sign https://www.compart.com/en/unicode/U+1018A
    }

    let mut fmt = String::new();
    let case = match case {
        Case::Lower => 0,
        Case::Upper => 1,
    };
    let thousands = [
        ["͵α", "͵Α"],
        ["͵β", "͵Β"],
        ["͵γ", "͵Γ"],
        ["͵δ", "͵Δ"],
        ["͵ε", "͵Ε"],
        ["͵ϛ", "͵Ϛ"],
        ["͵ζ", "͵Ζ"],
        ["͵η", "͵Η"],
        ["͵θ", "͵Θ"],
    ];
    let hundreds = [
        ["ρ", "Ρ"],
        ["σ", "Σ"],
        ["τ", "Τ"],
        ["υ", "Υ"],
        ["φ", "Φ"],
        ["χ", "Χ"],
        ["ψ", "Ψ"],
        ["ω", "Ω"],
        ["ϡ", "Ϡ"],
    ];
    let tens = [
        ["ι", "Ι"],
        ["κ", "Κ"],
        ["λ", "Λ"],
        ["μ", "Μ"],
        ["ν", "Ν"],
        ["ξ", "Ξ"],
        ["ο", "Ο"],
        ["π", "Π"],
        ["ϙ", "Ϟ"],
    ];
    let ones = [
        ["α", "Α"],
        ["β", "Β"],
        ["γ", "Γ"],
        ["δ", "Δ"],
        ["ε", "Ε"],
        ["ϛ", "Ϛ"],
        ["ζ", "Ζ"],
        ["η", "Η"],
        ["θ", "Θ"],
    ];
    // Extract a list of decimal digits from the number
    let mut decimal_digits: Vec<usize> = Vec::new();
    let mut n = n;
    while n > 0 {
        decimal_digits.push(n % 10);
        n /= 10;
    }

    // Pad the digits with leading zeros to ensure we can form groups of 4
    while decimal_digits.len() % 4 != 0 {
        decimal_digits.push(0);
    }
    decimal_digits.reverse();

    let mut M_power = decimal_digits.len() / 4 - 1;

    // M are used to represent 10000, M_power = 2 means 10000^2 = 10000 0000
    // The prefix of M is also made of Greek numerals but only be single digits, so it is 9 at max. This enables us
    // to represent up to (10000)^(9 + 1) - 1 = 10^40 -1  (9,999,999,999,999,999,999,999,999,999,999,999,999,999)
    let get_M_prefix = |M_power: usize| {
        if M_power == 0 {
            None
        } else {
            assert!(M_power <= 9);
            // the prefix of M is a single digit lowercase
            Some(ones[M_power - 1][0])
        }
    };

    let mut previous_has_number = false;
    for chunk in decimal_digits.chunks_exact(4) {
        // chunk must be exact 4 item
        assert_eq!(chunk.len(), 4);

        // `th`ousan, `h`undred, `t`en and `o`ne
        let (th, h, t, o) = (chunk[0], chunk[1], chunk[2], chunk[3]);
        if th + h + t + o == 0 {
            continue;
        }

        if previous_has_number {
            fmt.push_str(", ");
        }

        if let Some(m_prefix) = get_M_prefix(M_power) {
            fmt.push_str(m_prefix);
            fmt.push('Μ');
        }
        if th != 0 {
            let thousand_digit = thousands[th - 1][case];
            fmt.push_str(thousand_digit);
        }
        if h != 0 {
            let hundred_digit = hundreds[h - 1][case];
            fmt.push_str(hundred_digit);
        }
        if t != 0 {
            let ten_digit = tens[t - 1][case];
            fmt.push_str(ten_digit);
        }
        if o != 0 {
            let one_digit = ones[o - 1][case];
            fmt.push_str(one_digit);
        }
        // if we do not have thousan, we need to append 'ʹ' at the end.
        if th == 0 {
            fmt.push('ʹ');
        }
        if M_power > 0 {
            M_power = M_power.saturating_sub(1);
        }
        previous_has_number = true;
    }
    fmt
}

#[cfg(test)]
mod tests {
    use super::to_greek;
    use super::Case;

    macro_rules! greek_number_tests {
        ($($test_name:ident: $value:expr,)*) => {
            #[test]
            fn greek_number_stringify_test() {
                $(
                    {
                        let (number, string, case) = $value;
                        let s: String = to_greek(number, case).to_string();
                        assert_eq!(s, string, stringify!($test_name));
                    }
                )*
            }
        }
    }

    greek_number_tests! {
        single_digit_1_lower: (1, "αʹ", Case::Lower),
        single_digit_1_upper: (1, "Αʹ", Case::Upper),

        three_digit_241_lower: (241, "σμαʹ", Case::Lower),
        three_digit_241_upper: (241, "ΣΜΑʹ", Case::Upper),

        four_digit_5683_lower: (5683, "͵εχπγ", Case::Lower),
        four_digit_9184_lower: (9184, "͵θρπδ", Case::Lower),
        four_digit_3398_lower: (3398, "͵γτϙη", Case::Lower),
        four_digit_1005_lower: (1005, "͵αε", Case::Lower),

        long_complex_0: (97_554, "αΜθʹ, ͵ζφνδ", Case::Lower),
        long_complex_1: (2_056_839_184, "βΜκʹ, αΜ͵εχπγ, ͵θρπδ", Case::Lower),
        long_complex_2: (12_312_398_676, "βΜρκγʹ, αΜ͵ασλθ, ͵ηχοϛ", Case::Lower),

        trailing_high_digit_0: (2_000_000_000, "βΜκʹ", Case::Lower),
        trailing_high_digit_1: (90_000_001, "αΜ͵θ, αʹ", Case::Lower),
    }
}

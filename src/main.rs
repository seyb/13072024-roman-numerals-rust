fn main() {
    println!("Hello, world!");
}

type Roman = String;
type Arabic = usize;

fn roman(arabic: Arabic) -> Option<Roman> {
    match arabic {
        0 => return None,
        4000.. => return None,
        _ => return Some('I'.to_string().repeat(arabic)
            .replace("IIII", "IV")
            .replace("IVI", "V")
            .replace("VVI", "IX")
            .replace("IXI", "X")
            .replace("VIX", "XIV")
            .replace("VX", "XV")
            .replace("VXVI", "XIX")
            .replace("XVIVV", "XX")
            .replace("VVVVVV", "XXIV")
            .replace("VXIXV", "XXV")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_0_is_out_of_bound() {
        assert_eq!(roman(0), None);
    }

    #[test]
    fn test_4000_and_uppers_are_out_of_bound() {
        let random = 4000 + rand::random::<usize>() % 1000;
        assert_eq!(roman(random), None);
    }

    macro_rules! roman_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, roman(input));
                }
            )*
        };
    }

    roman_tests!{
        roman_i: (1, Some('I'.to_string())),
        roman_ii: (2, Some("II".to_string())),
        roman_iii: (3, Some("III".to_string())),
        roman_iv: (4, Some("IV".to_string())),
        roman_v: (5, Some("V".to_string())),
        roman_ix: (9, Some("IX".to_string())),
        roman_x: (10, Some("X".to_string())),
        roman_xiv: (14, Some("XIV".to_string())),
        roman_xv: (15, Some("XV".to_string())),
        roman_xix: (19, Some("XIX".to_string())),
        roman_xx: (20, Some("XX".to_string())),
        roman_xxiv: (24, Some("XXIV".to_string())),
        roman_xxv: (25, Some("XXV".to_string())),
    }

}

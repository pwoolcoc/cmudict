#![feature(try_from)]

use std::error;
use std::fmt;
use std::char;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub enum Stress {
    None,
    Primary,
    Secondary,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    AA(String, Stress),
    AE(String, Stress),
    AH(String, Stress),
    AO(String, Stress),
    AW(String, Stress),
    AY(String, Stress),
    B(String),
    CH(String),
    D(String),
    DH(String),
    EH(String, Stress),
    ER(String, Stress),
    EY(String, Stress),
    F(String),
    G(String),
    HH(String),
    IH(String, Stress),
    IY(String, Stress),
    JH(String),
    K(String),
    L(String),
    M(String),
    N(String),
    NG(String),
    OW(String, Stress),
    OY(String, Stress),
    P(String),
    R(String),
    S(String),
    SH(String),
    T(String),
    TH(String),
    UH(String, Stress),
    UW(String, Stress),
    V(String),
    W(String),
    Y(String),
    Z(String),
    ZH(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseError(ref s) => {
                s
            }
        }
    }
}

macro_rules! parse_stress {
    ( $next:expr, $symbol:expr, $name:expr ) => {{
        match $next {
            Some('0') | None => Ok($symbol($name, Stress::None)),
            Some('1') => Ok($symbol($name, Stress::Primary)),
            Some('2') => Ok($symbol($name, Stress::Secondary)),
            Some(_) => Err(Error::ParseError($name.to_string())),
        }
    }}
}

impl fmt::Display for Stress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Stress::None => write!(f, "{}", 0),
            Stress::Primary => write!(f, "{}", 1),
            Stress::Secondary => write!(f, "{}", 2),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol::AA(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AE(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AO(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::AY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::B(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::CH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::D(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::DH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::EH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::ER(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::EY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::F(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::G(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::HH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::IH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::IY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::JH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::K(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::L(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::M(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::N(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::NG(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::OW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::OY(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::P(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::R(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::S(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::SH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::T(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::TH(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::UH(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::UW(ref s1, ref s2) => {
                write!(f, "{}{}", s1, s2)
            },
            Symbol::V(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::W(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::Y(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::Z(ref s1) => {
                write!(f, "{}", s1)
            },
            Symbol::ZH(ref s1) => {
                write!(f, "{}", s1)
            },
        }
    }
}

impl<'a> TryFrom<&'a str> for Symbol {
    type Error = Error;

    fn try_from(s: &str) -> Result<Symbol, Error> {
        let mut chrs = s.chars();

        match chrs.next() {
            None => Err(Error::ParseError(s.into())),
            Some('A') => {
                match chrs.next() {
                    Some('A') => parse_stress!( chrs.next(), Symbol::AA, String::from(s) ),
                    Some('E') => parse_stress!( chrs.next(), Symbol::AE, String::from(s) ),
                    Some('H') => parse_stress!( chrs.next(), Symbol::AH, String::from(s) ),
                    Some('O') => parse_stress!( chrs.next(), Symbol::AO, String::from(s) ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::AW, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::AY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('E') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::EH, String::from(s) ),
                    Some('R') => parse_stress!( chrs.next(), Symbol::ER, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::EY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('I') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::IH, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::IY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('O') => {
                match chrs.next() {
                    Some('W') => parse_stress!( chrs.next(), Symbol::OW, String::from(s) ),
                    Some('Y') => parse_stress!( chrs.next(), Symbol::OY, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('U') => {
                match chrs.next() {
                    Some('H') => parse_stress!( chrs.next(), Symbol::UH, String::from(s) ),
                    Some('W') => parse_stress!( chrs.next(), Symbol::UW, String::from(s) ),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('B') => Ok(Symbol::B(String::from(s))),
            Some('C') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::CH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('D') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::DH(String::from(s))),
                    None => Ok(Symbol::D(String::from(s))),
                    Some(_) => Err(Error::ParseError(s.into())),
                }
            },
            Some('F') => Ok(Symbol::F(String::from(s))),
            Some('G') => Ok(Symbol::G(String::from(s))),
            Some('H') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::HH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('J') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::JH(String::from(s))),
                    Some(_) | None => Err(Error::ParseError(s.into())),
                }
            },
            Some('K') => Ok(Symbol::K(String::from(s))),
            Some('L') => Ok(Symbol::L(String::from(s))),
            Some('M') => Ok(Symbol::M(String::from(s))),
            Some('N') => {
                match chrs.next() {
                    Some('G') => Ok(Symbol::NG(String::from(s))),
                    None => Ok(Symbol::N(String::from(s))),
                    Some(_) => Err(Error::ParseError(s.into())),
                }
            },
            Some('P') => Ok(Symbol::P(String::from(s))),
            Some('R') => Ok(Symbol::R(String::from(s))),
            Some('S') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::SH(String::from(s))),
                    None => Ok(Symbol::S(String::from(s))),
                    Some(_) => Err(Error::ParseError(s.into())),
                }
            },
            Some('T') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::TH(String::from(s))),
                    None => Ok(Symbol::T(String::from(s))),
                    Some(_) => Err(Error::ParseError(s.into())),
                }
            },
            Some('V') => Ok(Symbol::V(String::from(s))),
            Some('W') => Ok(Symbol::W(String::from(s))),
            Some('Y') => Ok(Symbol::Y(String::from(s))),
            Some('Z') => {
                match chrs.next() {
                    Some('H') => Ok(Symbol::ZH(String::from(s))),
                    None => Ok(Symbol::Z(String::from(s))),
                    Some(_) => Err(Error::ParseError(s.into())),
                }
            },
            Some(_) => Err(Error::ParseError(s.into())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    label: String,
    pronounciation: Vec<Symbol>,
}


impl Rule {
    pub fn new(label: String, pronounciation: Vec<Symbol>) -> Rule {
        Rule {
            label: label,
            pronounciation: pronounciation,
        }
    }

    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
}

impl<'a> TryFrom<&'a str> for Rule {
    type Error = Error;

    fn try_from(s: &str) -> Result<Rule, Error> {
        let mut parts = s.split(char::is_whitespace);
        let label = match parts.next() {
            Some(label) => label,
            None => return Err(Error::ParseError(s.into())),
        };
        let parts = parts.filter(|s| s.len() > 0);
        let mut symbols = Vec::with_capacity(parts.size_hint().0);
        for part in parts {
            symbols.push(Symbol::try_from(part)?);
        }
        Ok(Rule {
            label: label.into(),
            pronounciation: symbols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Symbol, Stress, Rule};
    use std::convert::TryFrom;

    #[test]
    fn test_consonant() {
        let dh = "DH";
        let converted = Symbol::try_from(dh);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::DH("DH".to_string()));
    }

    #[test]
    fn test_vowel() {
        let aa = "AA1";
        let converted = Symbol::try_from(aa);
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), Symbol::AA("AA1".to_string(), Stress::Primary));
    }

    #[test]
    fn test_vec() {
        let v = vec!["AA1", "K", "L", "TH"];
        let converted = v.iter().map(|s| Symbol::try_from(s).unwrap()).collect::<Vec<_>>();
        assert_eq!(
                converted,
                vec![
                    Symbol::AA("AA1".to_string(), Stress::Primary),
                    Symbol::K("K".to_string()),
                    Symbol::L("L".to_string()),
                    Symbol::TH("TH".to_string()),
                ]
        );
    }

    #[test]
    fn test_rule() {
        // Slightly arbitrary, but tests the first 100 entries in the cmudict-0.7b. We do a
        // full test in the `cmudict` crate
        let rules: [&'static str; 100] = [
            r#"!EXCLAMATION-POINT  EH2 K S K L AH0 M EY1 SH AH0 N P OY2 N T"#,
            r#""CLOSE-QUOTE  K L OW1 Z K W OW1 T"#,
            r#""DOUBLE-QUOTE  D AH1 B AH0 L K W OW1 T"#,
            r#""END-OF-QUOTE  EH1 N D AH0 V K W OW1 T"#,
            r#""END-QUOTE  EH1 N D K W OW1 T"#,
            r#""IN-QUOTES  IH1 N K W OW1 T S"#,
            r#""QUOTE  K W OW1 T"#,
            r#""UNQUOTE  AH1 N K W OW1 T"#,
            r#"#HASH-MARK  HH AE1 M AA2 R K"#,
            r#"#POUND-SIGN  P AW1 N D S AY2 N"#,
            r#"#SHARP-SIGN  SH AA1 R P S AY2 N"#,
            r#"%PERCENT  P ER0 S EH1 N T"#,
            r#"&AMPERSAND  AE1 M P ER0 S AE2 N D"#,
            r#"'ALLO  AA2 L OW1"#,
            r#"'APOSTROPHE  AH0 P AA1 S T R AH0 F IY0"#,
            r#"'BOUT  B AW1 T"#,
            r#"'CAUSE  K AH0 Z"#,
            r#"'COURSE  K AO1 R S"#,
            r#"'CUSE  K Y UW1 Z"#,
            r#"'EM  AH0 M"#,
            r#"'END-INNER-QUOTE  EH1 N D IH1 N ER0 K W OW1 T"#,
            r#"'END-QUOTE  EH1 N D K W OW1 T"#,
            r#"'FRISCO  F R IH1 S K OW0"#,
            r#"'GAIN  G EH1 N"#,
            r#"'INNER-QUOTE  IH1 N ER0 K W OW1 T"#,
            r#"'KAY  K EY1"#,
            r#"'M  AH0 M"#,
            r#"'N  AH0 N"#,
            r#"'QUOTE  K W OW1 T"#,
            r#"'ROUND  R AW1 N D"#,
            r#"'S  EH1 S"#,
            r#"'SINGLE-QUOTE  S IH1 NG G AH0 L K W OW1 T"#,
            r#"'TIL  T IH1 L"#,
            r#"'TIS  T IH1 Z"#,
            r#"'TWAS  T W AH1 Z"#,
            r#"(BEGIN-PARENS  B IH0 G IH1 N P ER0 EH1 N Z"#,
            r#"(IN-PARENTHESES  IH1 N P ER0 EH1 N TH AH0 S IY2 Z"#,
            r#"(LEFT-PAREN  L EH1 F T P ER0 EH1 N"#,
            r#"(OPEN-PARENTHESES  OW1 P AH0 N P ER0 EH1 N TH AH0 S IY2 Z"#,
            r#"(PAREN  P ER0 EH1 N"#,
            r#"(PARENS  P ER0 EH1 N Z"#,
            r#"(PARENTHESES  P ER0 EH1 N TH AH0 S IY2 Z"#,
            r#")CLOSE-PAREN  K L OW1 Z P ER0 EH1 N"#,
            r#")CLOSE-PARENTHESES  K L OW1 Z P ER0 EH1 N TH AH0 S IY2 Z"#,
            r#")END-PAREN  EH1 N D P ER0 EH1 N"#,
            r#")END-PARENS  EH1 N D P ER0 EH1 N Z"#,
            r#")END-PARENTHESES  EH1 N D P ER0 EH1 N TH AH0 S IY2 Z"#,
            r#")END-THE-PAREN  EH1 N D DH AH0 P ER0 EH1 N"#,
            r#")PAREN  P ER0 EH1 N"#,
            r#")PARENS  P ER0 EH1 N Z"#,
            r#")RIGHT-PAREN  R AY1 T P EH2 R AH0 N"#,
            r#")UN-PARENTHESES  AH1 N P ER0 EH1 N TH AH0 S IY1 Z"#,
            r#"+PLUS  P L UH1 S"#,
            r#",COMMA  K AA1 M AH0"#,
            r#"--DASH  D AE1 SH"#,
            r#"-DASH  D AE1 SH"#,
            r#"-HYPHEN  HH AY1 F AH0 N"#,
            r#"...ELLIPSIS  IH2 L IH1 P S IH0 S"#,
            r#".DECIMAL  D EH1 S AH0 M AH0 L"#,
            r#".DOT  D AA1 T"#,
            r#".FULL-STOP  F UH1 L S T AA1 P"#,
            r#".PERIOD  P IH1 R IY0 AH0 D"#,
            r#".POINT  P OY1 N T"#,
            r#"/SLASH  S L AE1 SH"#,
            r#"3-D  TH R IY1 D IY2"#,
            r#"3D  TH R IY1 D IY2"#,
            r#":COLON  K OW1 L AH0 N"#,
            r#";SEMI-COLON  S EH1 M IY0 K OW1 L AH0 N"#,
            r#";SEMI-COLON(1)  S EH1 M IH0 K OW2 L AH0 N"#,
            r#"?QUESTION-MARK  K W EH1 S CH AH0 N M AA1 R K"#,
            r#"A  AH0"#,
            r#"A(1)  EY1"#,
            r#"A'S  EY1 Z"#,
            r#"A.  EY1"#,
            r#"A.'S  EY1 Z"#,
            r#"A.S  EY1 Z"#,
            r#"A42128  EY1 F AO1 R T UW1 W AH1 N T UW1 EY1 T"#,
            r#"AA  EY2 EY1"#,
            r#"AAA  T R IH2 P AH0 L EY1"#,
            r#"AABERG  AA1 B ER0 G"#,
            r#"AACHEN  AA1 K AH0 N"#,
            r#"AACHENER  AA1 K AH0 N ER0"#,
            r#"AAH  AA1"#,
            r#"AAKER  AA1 K ER0"#,
            r#"AALIYAH  AA2 L IY1 AA2"#,
            r#"AALSETH  AA1 L S EH0 TH"#,
            r#"AAMODT  AA1 M AH0 T"#,
            r#"AANCOR  AA1 N K AO2 R"#,
            r#"AARDEMA  AA0 R D EH1 M AH0"#,
            r#"AARDVARK  AA1 R D V AA2 R K"#,
            r#"AARDVARKS  AA1 R D V AA2 R K S"#,
            r#"AARGH  AA1 R G"#,
            r#"AARON  EH1 R AH0 N"#,
            r#"AARON'S  EH1 R AH0 N Z"#,
            r#"AARONS  EH1 R AH0 N Z"#,
            r#"AARONSON  EH1 R AH0 N S AH0 N"#,
            r#"AARONSON(1)  AA1 R AH0 N S AH0 N"#,
            r#"AARONSON'S  EH1 R AH0 N S AH0 N Z"#,
            r#"AARONSON'S(1)  AA1 R AH0 N S AH0 N Z"#,
            r#"AARTI  AA1 R T IY2"#,
        ];
        for rule in rules.iter() {
            let r = Rule::try_from(rule).expect(&format!("Could not parse rule '{}'", rule));
        }
    }
}

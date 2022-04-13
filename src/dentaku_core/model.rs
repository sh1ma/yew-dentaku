use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative
}

#[derive(Debug, PartialEq, Clone)]
pub struct InputtableNumber {
    pub value: i32
}
// pub enum InputtableNumber {
//     Zero,
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine
// }

impl fmt::Display for InputtableNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// impl fmt::Display for InputtableNumber {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             InputtableNumber::Zero => write!(f, "0"),
//             InputtableNumber::One => write!(f, "1"),
//             InputtableNumber::Two => write!(f, "2"),
//             InputtableNumber::Three => write!(f, "3"),
//             InputtableNumber::Four => write!(f, "4"),
//             InputtableNumber::Five => write!(f, "5"),
//             InputtableNumber::Six => write!(f, "6"),
//             InputtableNumber::Seven => write!(f, "7"),
//             InputtableNumber::Eight => write!(f, "8"),
//             InputtableNumber::Nine => write!(f, "9"),
//         }
//     }
// }
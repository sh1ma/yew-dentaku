// use crate::model::{Sign, InputtableNumber};

use std::fmt;

use super::model::{InputtableNumber, Sign};
use log;

#[derive(Debug, Clone, PartialEq)]
pub struct DentakuInputState {
    pub value: String,
    sign: Sign,
    has_decimal: bool,
    decimal_point_index: usize, 
}

impl Default for DentakuInputState {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            sign: Sign::Positive,
            has_decimal: false,
            decimal_point_index: usize::default()
        }
    }
}

impl fmt::Display for DentakuInputState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-").expect("unexcepted error");
        }
        if self.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl DentakuInputState {
    pub fn is_empty(self: &Self) -> bool {
        self.value.is_empty()
    }

    pub fn update_value(&self, value: String) -> Self {
        Self {
            value: value,
            sign: self.sign.clone(),
            has_decimal: self.has_decimal,
            decimal_point_index: self.decimal_point_index
        }
    }

    pub fn update_sign(&self, sign: Sign) -> Self {
        Self {
            value: self.value.clone(),
            sign: sign,
            has_decimal: self.has_decimal,
            decimal_point_index: self.decimal_point_index
        }
    }

    pub fn update_has_decimal(&self, has_decimal: bool) -> Self {
        Self {
            value: self.value.clone(),
            sign: self.sign.clone(),
            has_decimal: has_decimal,
            decimal_point_index: self.decimal_point_index
        }
    }

    pub fn update_decimal_point_index(&self, decimal_point_index: usize) -> Self {
        Self {
            value: self.value.clone(),
            sign: self.sign.clone(),
            has_decimal: self.has_decimal,
            decimal_point_index: decimal_point_index
        }
    }

    pub fn try_toggle_sign(&self, sign: Sign) -> Result<Self, String> {
        if self.sign == sign {
            let current_sign_str = if self.sign == Sign::Positive { "positive" } else { "negative" };
            Err(format!("Sign is already {}", current_sign_str))
        } else {
            Ok(self.update_sign(sign))
        }
    }

    pub fn try_add_digit(&self, number: InputtableNumber) -> Result<Self, String> {
        if self.is_empty() && number.value == 0 {
            log::info!("Update1: {:?}", number.value);
            Ok(self.clone())
        } else {

            let updated_number_string = format!("{}{}", self.value, number.value);
            Ok(self.update_value(updated_number_string))
        }
    }

    pub fn try_add_decimal_point(&self) -> Result<Self, String> {
        if self.has_decimal {
            return Err("the value already has decimal value".to_string());
        }

        let current_value = if self.is_empty() {
            "0".to_string()
        } else {
            self.value.clone()
        };

        Ok(self
        .update_decimal_point_index(current_value.len() as usize)
        .update_value(format!("{}{}", current_value, "."))
        .update_has_decimal(true))
    }

    pub fn to_f32(&self) -> f32 {
        self.value.parse::<f32>().expect("unexcepted error")
    }
}
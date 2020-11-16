use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::ops::Index;

pub type Settings = Vec<Setting>;

#[derive(Debug)]
pub struct Setting {
    name: String,
    value: SettingOption,
}

#[derive(Debug)]
pub enum SettingOption {
    CustomStr(CustomStrOption),
    StrList(StrListOption),
    Float(FloatOption),
    Integer(IntOption),
}

pub type Validator = Box<dyn Validate>;

#[derive(Debug)]
pub struct CustomStrOption {
    value: String,
    validator: Option<Validator>,
}

impl CustomStrOption {
    pub fn new(value: String, validator: Option<Validator>) -> Result<Self, ValidateError> {
        if let Some(validator) = &validator {
            if let Err(e) = validator.validate(&value) {
                return Err(e);
            }
        }
        Ok(Self { value, validator })
    }

    pub fn rules(&self) -> &str {
        match &self.validator {
            None => "",
            Some(v) => v.rules(),
        }
    }

    pub fn validate(&self, value: &str) -> Result<(), ValidateError> {
        match &self.validator {
            None => Ok(()),
            Some(validator) => validator.validate(value),
        }
    }

    pub fn current_value(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Debug)]
pub struct SetOptionError<T: Debug> {
    error: ValidateError,
    value: T,
}

impl<T: Debug> SetOptionError<T> {
    pub fn new(error: ValidateError, value: T) -> Self {
        Self { error, value }
    }

    pub fn take_value(self) -> T {
        self.value
    }
}

impl<T: Debug> Error for SetOptionError<T> {}

impl<T: Debug> fmt::Display for SetOptionError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can't set option: {}", self.error)
    }
}

pub trait Validate: Debug {
    fn rules(&self) -> &str {
        ""
    }

    fn validate(&self, value: &str) -> Result<(), ValidateError>;
}

#[derive(Debug)]
pub struct ValidateError {
    msg: String,
}

impl ValidateError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Error for ValidateError {}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validation fails: {}", self.msg)
    }
}

#[derive(Debug)]
pub struct StrListOption {
    value: usize,
    variants: Vec<String>,
}

impl StrListOption {
    pub fn new(value: usize, variants: Vec<String>) -> Result<Self, ValidateError> {
        Self::validate_inner(value, &variants).map(|_| Self { value, variants })
    }

    pub fn variants_len(&self) -> usize {
        self.variants.len()
    }

    pub fn current_value(&self) -> usize {
        self.value
    }

    pub fn validate(&self, value: usize) -> Result<(), ValidateError> {
        Self::validate_inner(value, &self.variants)
    }

    pub fn validate_inner(value: usize, variants: &[String]) -> Result<(), ValidateError> {
        if value >= variants.len() {
            return Err(ValidateError::new(Self::error_message(
                value,
                variants.len(),
            )));
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.variants.iter()
    }

    fn error_message(value: usize, variants_len: usize) -> String {
        format!(
            "Value must be valid variants index. Variants length: {}. Value: {}",
            variants_len, value
        )
    }
}

impl Index<usize> for StrListOption {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.variants[index]
    }
}

#[derive(Debug)]
pub struct FloatOption {
    value: f32,
    min: f32,
    max: f32,
}

impl FloatOption {
    pub fn new(min: f32, max: f32, value: f32) -> Result<Self, ValidateError> {
        Self::validate_inner(min, max, value).map(|_| Self { min, max, value })
    }

    fn validate_inner(min: f32, max: f32, value: f32) -> Result<(), ValidateError> {
        if min > max {
            let msg = format!(
                "FloatOption requires min <= max. Min: {}; Max: {}",
                min, max
            );
            return Err(ValidateError::new(msg));
        }
        if value < min || value > max {
            let msg = format!("Value must be in [{}, {}]. Value: {}", min, max, value);
            return Err(ValidateError::new(msg));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValidateError> {
        Self::validate_inner(self.min, self.max, self.value)
    }
}

#[derive(Debug)]
pub struct IntOption {
    value: i64,
    min: i64,
    max: i64,
}

impl IntOption {
    pub fn new(min: i64, max: i64, value: i64) -> Result<Self, ValidateError> {
        Self::validate_inner(min, max, value).map(|_| Self { min, max, value })
    }

    fn validate_inner(min: i64, max: i64, value: i64) -> Result<(), ValidateError> {
        if min > max {
            let msg = format!("IntOption requires min <= max. Min: {}; Max: {}", min, max);
            return Err(ValidateError::new(msg));
        }
        if value < min || value > max {
            let msg = format!(
                "IntOption value must be in [{}, {}]. Value: {}",
                min, max, value
            );
            return Err(ValidateError::new(msg));
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValidateError> {
        Self::validate_inner(self.min, self.max, self.value)
    }
}

use std::{fmt, ops::Deref};

use protobuf::descriptor;

//  The name of the uninterpreted option.  Each string represents a segment in
///  a dot-separated name.
///
///  E.g.,`{ ["foo", false], ["bar.baz", true], ["qux", false] }` represents
///  `"foo.(bar.baz).qux"`.
#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
pub struct NamePart {
    pub value: Box<str>,
    pub formatted_value: Box<str>,
    pub is_extension: bool,
}

impl NamePart {
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
    /// true if a segment represents an extension (denoted with parentheses in
    ///  options specs in .proto files).
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        self.is_extension
    }

    /// Returns the formatted value of the `NamePart`
    ///
    /// If `is_extension` is `true`, the formatted value will be wrapped in
    /// parentheses.
    #[must_use]
    pub fn formatted_value(&self) -> &str {
        &self.formatted_value
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for NamePart {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NamePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_extension {
            write!(f, "({})", self.value())
        } else {
            write!(f, "{}", self.value())
        }
    }
}

impl From<descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: descriptor::uninterpreted_option::NamePart) -> Self {
        let is_extension = part.is_extension.unwrap_or(false);
        let value: Box<str> = part.name_part.unwrap_or_default().into();
        let formatted_value = if is_extension {
            format!("({})", &value).into()
        } else {
            value.clone()
        };
        Self {
            is_extension,
            value,
            formatted_value,
        }
    }
}

impl From<&descriptor::uninterpreted_option::NamePart> for NamePart {
    fn from(part: &descriptor::uninterpreted_option::NamePart) -> Self {
        Self::from(part.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NameParts {
    parts: Vec<NamePart>,
}

impl std::fmt::Display for NameParts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl<'a> std::iter::IntoIterator for &'a NameParts {
    type Item = &'a NamePart;
    type IntoIter = std::slice::Iter<'a, NamePart>;
    fn into_iter(self) -> Self::IntoIter {
        self.parts.iter()
    }
}

impl NameParts {
    pub fn iter(&self) -> std::slice::Iter<'_, NamePart> {
        self.parts.iter()
    }
    #[must_use]
    pub fn get(&self, idx: usize) -> Option<&NamePart> {
        self.parts.get(idx)
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.parts.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    #[must_use]
    pub fn contains(&self, part: &str) -> bool {
        self.parts.iter().any(|p| p.value() == part)
    }
    #[must_use]
    pub fn formatted(&self) -> String {
        itertools::join(self.iter().map(|v| v.formatted_value()), ".")
    }
}

pub struct UninterpretedOptions {
    options: Vec<UninterpretedOption>,
}
impl Deref for UninterpretedOptions {
    type Target = [UninterpretedOption];
    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

/// A message representing an option that parser does not recognize.
#[derive(Debug, Clone, PartialEq)]
pub struct UninterpretedOption {
    name: Box<[NamePart]>,
    value: UninterpretedValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UninterpretedValue {
    Identifier(String),
    PositiveInt(u64),
    NegativeInt(i64),
    Double(f64),
    String(Vec<u8>),
    Aggregate(String),
}

impl UninterpretedValue {
    /// Returns `true` if the uninterpreted option value is [`Identifier`].
    ///
    /// [`Identifier`]: UninterpretedOptionValue::Identifier
    #[must_use]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(..))
    }

    #[must_use]
    pub fn as_identifier(&self) -> Option<&String> {
        if let Self::Identifier(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the uninterpreted option value is [`PositiveInt`].
    ///
    /// [`PositiveInt`]: UninterpretedOptionValue::PositiveInt
    #[must_use]
    pub fn is_positive_int(&self) -> bool {
        matches!(self, Self::PositiveInt(..))
    }

    #[must_use]
    pub fn as_positive_int(&self) -> Option<&u64> {
        if let Self::PositiveInt(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_positive_int(self) -> Result<u64, Self> {
        if let Self::PositiveInt(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn try_into_identifier(self) -> Result<String, Self> {
        if let Self::Identifier(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the uninterpreted option value is [`NegativeInt`].
    ///
    /// [`NegativeInt`]: UninterpretedOptionValue::NegativeInt
    #[must_use]
    pub fn is_negative_int(&self) -> bool {
        matches!(self, Self::NegativeInt(..))
    }

    #[must_use]
    pub fn as_negative_int(&self) -> Option<&i64> {
        if let Self::NegativeInt(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_negative_int(self) -> Result<i64, Self> {
        if let Self::NegativeInt(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the uninterpreted option value is [`Double`].
    ///
    /// [`Double`]: UninterpretedOptionValue::Double
    #[must_use]
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(..))
    }

    #[must_use]
    pub fn as_double(&self) -> Option<&f64> {
        if let Self::Double(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_double(self) -> Result<f64, Self> {
        if let Self::Double(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the uninterpreted option value is [`String`].
    ///
    /// [`String`]: UninterpretedOptionValue::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    #[must_use]
    pub fn as_string(&self) -> Option<&Vec<u8>> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_string(self) -> Result<Vec<u8>, Self> {
        if let Self::String(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the uninterpreted option value is [`Aggregate`].
    ///
    /// [`Aggregate`]: UninterpretedOptionValue::Aggregate
    #[must_use]
    pub fn is_aggregate(&self) -> bool {
        matches!(self, Self::Aggregate(..))
    }

    #[must_use]
    pub fn as_aggregate(&self) -> Option<&String> {
        if let Self::Aggregate(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn try_into_aggregate(self) -> Result<String, Self> {
        if let Self::Aggregate(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl From<descriptor::UninterpretedOption> for UninterpretedOption {
    fn from(option: descriptor::UninterpretedOption) -> Self {
        let descriptor::UninterpretedOption {
            name,
            identifier_value,
            negative_int_value,
            double_value,
            string_value,
            aggregate_value,
            positive_int_value,
            special_fields: _,
        } = option;

        let name = name.into_iter().map(Into::into).collect::<Box<[_]>>();

        let value = if let Some(value) = identifier_value {
            UninterpretedValue::Identifier(value)
        } else if let Some(value) = positive_int_value {
            UninterpretedValue::PositiveInt(value)
        } else if let Some(value) = negative_int_value {
            UninterpretedValue::NegativeInt(value)
        } else if let Some(value) = double_value {
            UninterpretedValue::Double(value)
        } else if let Some(value) = string_value {
            UninterpretedValue::String(value)
        } else if let Some(value) = aggregate_value {
            UninterpretedValue::Aggregate(value)
        } else {
            UninterpretedValue::PositiveInt(0)
        };
        Self { name, value }
    }
}

impl UninterpretedOption {
    #[must_use]
    pub fn name(&self) -> &[NamePart] {
        self.name.as_ref()
    }

    #[must_use]
    pub const fn value(&self) -> &UninterpretedValue {
        &self.value
    }
}

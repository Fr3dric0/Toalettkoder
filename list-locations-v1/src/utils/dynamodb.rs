use aws_sdk_dynamodb::types::AttributeValue;
use std::str::FromStr;

pub fn as_string(value: Option<&AttributeValue>) -> Option<String> {
    if let Some(attribute) = value {
        attribute.as_s().map(|val| val.to_string()).ok()
    } else {
        None
    }
}

pub fn as_number<T: FromStr>(value: Option<&AttributeValue>) -> Option<T> {
    match value {
        Some(attribute) => attribute
            .as_n()
            .map(|number| str::parse::<T>(number))
            .unwrap()
            .ok(),
        None => None,
    }
}

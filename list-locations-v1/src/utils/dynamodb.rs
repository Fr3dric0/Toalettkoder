use aws_sdk_dynamodb::types::AttributeValue;

pub fn as_string(value: Option<&AttributeValue>) -> Option<String> {
    if let Some(attribute) = value {
        attribute.as_s().map(|val| val.to_string()).ok()
    } else {
        None
    }
}

pub fn as_float(value: Option<&AttributeValue>) -> Option<f64> {
    match value {
        Some(attribute) => attribute
            .as_n()
            .map(|number| str::parse(number).unwrap())
            .ok(),
        None => None,
    }
}

pub fn as_unsigned_int(value: Option<&AttributeValue>) -> Option<u32> {
    match value {
        Some(attribute) => attribute
            .as_n()
            .map(|number| str::parse(number).unwrap())
            .ok(),
        None => None,
    }
}

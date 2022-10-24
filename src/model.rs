pub struct Data {
    date: String,
    value: String,
    id: String,
    description: String,
}

impl ToString for Data {
    fn to_string(&self) -> String {
        return format!(
            "The {} has value {} in {} with description: {}",
            &self.id, &self.value, &self.date, &self.description
        );
    }
}

pub fn convert_to_data(value_data: Vec<&str>) -> Data {
    let data = Data {
        date: value_data[0].to_string(),
        value: value_data[1].to_string(),
        id: value_data[2].to_string(),
        description: value_data[3].to_string(),
    };
    return data;
}

pub fn split_pattern_file() -> String {
    return String::from(",");
}

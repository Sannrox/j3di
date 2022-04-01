pub mod edit_json {

    use crate::edit::update_value;
    use crate::Types;
    use serde_json::Number;
    use serde_json::Value;

    pub fn update(readed_data: &mut Value, update: String, value: String, type_of: Types) {
        let new_value: Vec<String> = value.split(',').map(|s| s.to_string()).collect();
        let json_path: Vec<String> = update.split('.').map(|s| s.to_string()).collect();
        recursive_json_tree(json_path, readed_data, &new_value, type_of);
    }

    pub(self) fn recursive_json_tree<'a, T: serde_json::value::Index>(
        mut vector: Vec<T>,
        object: &'a mut serde_json::Value,
        wert: &'a Vec<String>,
        type_of: Types,
    ) where
        T: std::clone::Clone + std::fmt::Debug,
    {
        if vector.len() > 1 {
            let first_ele = vector[0].clone();
            vector.remove(0);

            recursive_json_tree(vector.to_vec(), &mut object[first_ele], wert, type_of)
        } else {
            match type_of {
                Types::String => {
                    if wert.len() == 1 {
                        let thing: String = wert.first().unwrap().to_string();
                        update_value::to_string(vector, object, &thing)
                    }
                }
                Types::Array => update_value::to_array(
                    vector,
                    object,
                    wert.iter()
                        .map(|s| serde_json::to_value(s).unwrap())
                        .collect(),
                ),
                Types::Null => update_value::to_null(vector, object),
                Types::Number => {
                    if wert.len() == 1 {
                        let thing = wert.first().unwrap().to_string().parse::<f64>().unwrap();
                        if let Some(number) = Number::from_f64(thing) {
                            update_value::to_number(vector, object, number)
                        }
                    }
                }
                Types::Object => {
                    let thing = wert.first().unwrap().to_string();
                    let test_array: serde_json::Map<String, Value> =
                        serde_json::from_str(&thing).unwrap();
                    update_value::to_object(vector, object, test_array)
                }
            }
        }
    }
}

pub mod update_value {
    use serde_json::Value;

    pub fn to_string<T: serde_json::value::Index>(
        vector: Vec<T>,
        object: &mut serde_json::Value,
        wert: &String,
    ) {
        let last = &vector[0];
        let new_value = Value::String(wert.to_string());
        object[last] = new_value;
    }

    #[test]
    fn string() {
        let vector = vec!["name"];
        let wert = String::from("Jane Doe");

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let ref_data = r#"
        {
            "name": "Jane Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let mut v: Value = serde_json::from_str(data).unwrap();
        let ref_v: Value = serde_json::from_str(ref_data).unwrap();

        to_string(vector, &mut v, &wert);

        assert_eq!(v, ref_v)
    }

    pub fn to_array<T: serde_json::value::Index>(
        vector: Vec<T>,
        object: &mut serde_json::Value,
        wert: Vec<Value>,
    ) {
        let last = &vector[0];
        let new_value = Value::Array(wert);
        object[last] = new_value;
    }

    #[test]
    fn array() {
        let vector = vec!["name"];
        let array = r#"["Jane Doe", "Francis Doe", "Matt Doe"]"#;

        let test_array: Vec<Value> = serde_json::from_str(array).unwrap();

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let ref_data = r#"
        {
            "name": ["Jane Doe", "Francis Doe", "Matt Doe"],
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let mut v: Value = serde_json::from_str(data).unwrap();
        let ref_v: Value = serde_json::from_str(ref_data).unwrap();

        to_array(vector, &mut v, test_array);

        assert_eq!(v, ref_v)
    }

    pub fn to_object<T: serde_json::value::Index>(
        vector: Vec<T>,
        object: &mut serde_json::Value,
        wert: serde_json::Map<String, Value>,
    ) {
        let last = &vector[0];
        let new_value = Value::Object(wert);
        object[last] = new_value;
    }

    #[test]
    fn object() {
        let vector = vec!["name"];
        let array = r#"{ "Forename": "Jane", "Surname": "Doe"}"#;

        let test_array: serde_json::Map<String, Value> = serde_json::from_str(array).unwrap();

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let ref_data = r#"
        {
            "name":{ "Forename": "Jane", "Surname": "Doe"},
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let mut v: Value = serde_json::from_str(data).unwrap();
        let ref_v: Value = serde_json::from_str(ref_data).unwrap();

        to_object(vector, &mut v, test_array);

        assert_eq!(v, ref_v)
    }

    pub fn to_number<T: serde_json::value::Index>(
        vector: Vec<T>,
        object: &mut serde_json::Value,
        wert: serde_json::Number,
    ) {
        let last = &vector[0];
        let new_value = Value::Number(wert);
        object[last] = new_value;
    }

    #[test]
    fn number() {
        let vector = vec!["age"];
        let number = r#"15"#;

        let test_number: serde_json::Number = serde_json::from_str(number).unwrap();

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let ref_data = r#"
        {
            "name": "John Doe",
            "age": 15,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let mut v: Value = serde_json::from_str(data).unwrap();
        let ref_v: Value = serde_json::from_str(ref_data).unwrap();

        to_number(vector, &mut v, test_number);

        assert_eq!(v, ref_v)
    }

    pub fn to_null<T: serde_json::value::Index>(vector: Vec<T>, object: &mut serde_json::Value) {
        let last = &vector[0];
        let new_value = Value::Null;
        object[last] = new_value;
    }

    #[test]
    fn null() {
        let vector = vec!["phones"];

        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let ref_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": null
        }"#;

        let mut v: Value = serde_json::from_str(data).unwrap();
        let ref_v: Value = serde_json::from_str(ref_data).unwrap();

        to_null(vector, &mut v);

        assert_eq!(v, ref_v)
    }
}

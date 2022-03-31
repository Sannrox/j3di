pub mod file_actions {

    use anyhow::{Context, Error, Result};
    use serde_json::Value;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    pub fn write_json_file(data: &Value, path: &Path, pretty: bool) {
        let output: String = if pretty {
            serde_json::to_string_pretty(&data)
                .with_context(|| format!("could not parse json: \n `{}`", &data.to_string()))
                .unwrap()
        } else {
            serde_json::to_string(&data)
                .with_context(|| format!("could not parse json: \n `{}`", &data.to_string()))
                .unwrap()
        };

        let mut file = File::create(path)
            .with_context(|| format!("could not read file `{}`", &path.display()))
            .unwrap();

        file.write_all(output.as_bytes())
            .expect("Could not write to file");
    }

    pub fn read_json_file(path: &Path) -> Value {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("could not read file `{}`", &path.display()))
            .unwrap();

        serde_json::from_str(&content)
            .with_context(|| format!("could not parse json: \n `{}`", &content.to_string()))
            .unwrap()
    }
}

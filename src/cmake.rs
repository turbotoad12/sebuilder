use regex::Regex;
use std::error::Error;
use std::fs;

/// Updates a CMake `set(VAR "value" CACHE ...)` line.
/// Example:
/// set(SE_APP_NAME "Old" CACHE STRING "desc")
/// becomes:
/// set(SE_APP_NAME "NewValue" CACHE STRING "desc")
pub fn update_cmake_set(path: &str, var_name: &str, new_value: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    // Regex to match:
    // set(SE_APP_NAME "anything here" CACHE ...
    let pattern = format!(r#"set\(\s*{}\s*"([^"]*)"(.*?)\)"#, regex::escape(var_name));

    let re = Regex::new(&pattern)?;

    // Replace only the value inside the quotes
    let replaced = re.replace(&content, |caps: &regex::Captures| {
        format!(r#"set({} "{}" {})"#, var_name, new_value, &caps[2])
    });

    fs::write(path, replaced.as_ref())?;
    Ok(())
}

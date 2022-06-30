/*
    Code always comes encapsulated in
    markdown code blocks. This function
    extracts the code from the block
*/
pub fn unwrap_code(raw_data: &str) -> Result<String, String> {
    if raw_data.chars().take(6).collect::<String>() != "```lua" {
        return Err("Could not parse code block, please check the documentation on how to format your submissions".to_string());
    }
    Ok(raw_data.chars().skip(6).take_while(|c| *c != '`').collect())
}

#[cfg(test)]
mod tests {
    use crate::code_unwrapper::unwrap_code;

    #[test]
    /// Test that code is unwrapped correctly
    fn code_unwrapper_test() {
        let raw_data = "```lua\nprint(\"Hello World\")\n```";
        let code = unwrap_code(raw_data).unwrap();
        assert_eq!(code, "\nprint(\"Hello World\")\n");
    }
}

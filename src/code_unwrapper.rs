
/*
	Code always comes encapsulated in 
	markdown code blocks. This function
	extracts the code from the block
*/
pub fn unwrap_code(raw_data: &str) -> Result<String, &str> {
	if raw_data.chars().take(6).collect::<String>() != "```lua" {
		return Err("Invalid submission, could not parse code block");
	}
	Ok(raw_data.chars().skip(6).take_while(|c| *c != '`').collect())
}
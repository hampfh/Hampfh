pub(crate) fn escape_string(string: String) -> String {
    return string
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "")
        .replace("        ", "\\t")
        .replace("\t", "\\t");
}

pub struct ParsedCommand {
    pub raw: String,
    pub name: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Option<ParsedCommand> {
    let mut parts = input.split_whitespace();

    let name = parts.next()?.to_string();

    let args = parts.map(str::to_string).collect();

    Some(ParsedCommand {
        raw: input.to_owned(),
        name,
        args,
    })
}

pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Continue,  // Milestone 2. Stopping, resuming, and restarting the inferior
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            // Milestone 2. Stopping, resuming, and restarting the inferior
            "c" | "cont" | "continue" => Some(DebuggerCommand::Continue),
            // Default case:
            _ => None,
        }
    }
}

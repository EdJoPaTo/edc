#[derive(Debug, PartialEq)]
pub struct Command {
    program: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: vec![],
        }
    }

    pub fn arg(&mut self, arg: &str) {
        self.args.push(arg.to_string());
    }

    pub fn args(&mut self, args: &[&str]) {
        for arg in args {
            self.arg(arg);
        }
    }

    pub fn to_command_line(&self) -> String {
        let mut line = String::new();
        line += &self.program;

        for arg in &self.args {
            if arg.contains(' ') {
                line += &format!(" \"{}\"", arg);
            } else {
                line += " ";
                line += &arg;
            }
        }

        line
    }

    pub fn to_std_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new(&self.program);
        command.args(&self.args);
        command
    }
}

#[test]
fn generates_command_line() {
    let mut command = Command::new("mkdir");
    command.arg("-p");
    command.arg("foo bar");
    assert_eq!(command.to_command_line(), "mkdir -p \"foo bar\"")
}

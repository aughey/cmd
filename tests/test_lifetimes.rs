use rusty_cmd::{cmd::Cmd, command_handler::CommandHandler};

#[test]
fn test_simple() -> std::io::Result<()> {
    let input = "quit";
    let stdin = std::io::BufReader::new(input.as_bytes());

    let stdout: Vec<u8> = Vec::new();
    let mut app = Cmd::new(stdin, stdout);
    app.add_cmd(String::from("quit"), rusty_cmd::handlers::Quit::default())?;
    app.run()?;
    Ok(())
}

#[test]
fn test_lifetime_req() -> std::io::Result<()> {
    let input = "quit";
    let stdin = std::io::BufReader::new(input.as_bytes());

    let goodbye_message = "Goodbye!".to_string();

    let stdout: Vec<u8> = Vec::new();
    let mut app = Cmd::new(stdin, stdout);
    app.add_cmd_fn(
        String::from("quit"),
        |stdout, _args| -> rusty_cmd::command_handler::CommandResult {
            stdout.extend_from_slice(goodbye_message.as_bytes());
            rusty_cmd::command_handler::CommandResult::Break
        },
    )?;
    app.run()?;
    Ok(())
}

#[test]
fn test_lifetime_struct() -> std::io::Result<()> {
    let input = "quit";
    let stdin = std::io::BufReader::new(input.as_bytes());

    let goodbye_message = "Goodbye!".to_string();

    struct CustomHandler<'b> {
        goodbye_message: &'b String,
    }
    impl<W> CommandHandler<W> for CustomHandler<'_>
    where
        W: std::io::Write,
    {
        fn execute(
            &self,
            stdout: &mut W,
            _args: &[&str],
        ) -> rusty_cmd::command_handler::CommandResult {
            stdout.write_all(self.goodbye_message.as_bytes()).unwrap();
            rusty_cmd::command_handler::CommandResult::Break
        }
    }

    let stdout: Vec<u8> = Vec::new();
    let mut app = Cmd::new(stdin, stdout);
    app.add_cmd(
        String::from("quit"),
        CustomHandler {
            goodbye_message: &goodbye_message,
        },
    )?;
    app.run()?;
    Ok(())
}

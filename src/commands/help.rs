use serenity::{ all::CommandDataOption, builder::{CreateCommand, CreateCommandOption} };

pub fn run( options: &[CommandDataOption] ) -> String {
    let help = options.first().unwrap();
    let value = help.value.as_str().unwrap();
    match value {
        "roll" => "Rolling value. Example /roll x-sided: d10 number: 1".to_owned(),
        _ => "Have to add command name.".to_owned()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new( "help" ).description( "Help for command this bot." )
        .set_options( vec![ CreateCommandOption::new( serenity::all::CommandOptionType::String, "command", "Shows information about the command." ) ] )
}
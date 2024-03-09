use serenity::{ all::CommandDataOption, builder::{CreateCommand, CreateCommandOption} };

pub fn run( options: &[CommandDataOption] ) -> String {
    if options.len() == 0 {
        return String::from( "Example /roll x-sided: d10 number: 1." );
    }
    let help = options.first().unwrap();
    let value = help.value.as_str().unwrap();
    match value {
        "roll" => "Random value. Physical dice such as d4, d6, d8, d10, d12, d20 and d100 are used to draw the values. Abstract dice that cannot physically exist, such as d2, d3, d5 and d50, have also been added.".to_owned(),
        _ => "Have to add command name.".to_owned()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new( "help" ).description( "Help for command this bot." )
        .set_options( vec![ CreateCommandOption::new( serenity::all::CommandOptionType::String, "command", "Shows information about the command." ) ] )
}
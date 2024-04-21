// Importing necessary modules and traits from the serenity crate.
use serenity::{ all::CommandDataOption, builder::{ CreateCommand, CreateCommandOption } };

// Function to handle the /help command, providing information about bot commands.
pub fn run( options: &[CommandDataOption] ) -> String {
    // Checking if the user provided any options.
    if options.len() == 0 {
        return String::from( "Example /roll x-sided: d10 number: 1." );
    }
    // Extracting the specified command from the options.
    let help = options.first().unwrap();
    let value = help.value.as_str().unwrap();
    // Providing information based on the specified command.
    match value {
        "roll" => "Random value. Physical dice such as d4, d6, d8, d10, d12, d20 and d100 are used to draw the values. Abstract dice that cannot physically exist, such as d2, d3, d5 and d50, have also been added.".to_owned(),
        _ => "Have to add command name.".to_owned()
    }
}

// Function to register the /help command with necessary options.
pub fn register() -> CreateCommand {
    CreateCommand::new( "help" ).description( "Help for command this bot." )
        .set_options( vec![ CreateCommandOption::new( serenity::all::CommandOptionType::String, "command", "Shows information about the command." ) ] )
}
// Importing necessary modules and traits from the standard library and external crates.
use rand::Rng;
use serenity::{ all::CommandDataOption, builder::{ CreateCommand, CreateCommandOption } };

// Function to handle the /roll command, which generates random numbers based on the specified dice and quantity.
pub fn run( options: &[CommandDataOption] ) -> String {
    // Checking if the user provided necessary options for the command.
    if options.len() < 1 {
        return format!( "/roll x-sided counter" );
    }
    // Extracting the specified dice type and quantity from the options.
    let roll = options[0].value.as_str().unwrap();
    let number;
    if options.len() < 2 {
        number = options[1].value.as_i64().unwrap();
    } else {
        number = 1;
    }
    // Rolling the dice and calculating the result.
    let num = my_match( roll );
    let mut result = num.to_string();
    let mut counter = num;
    for _n in 1..number {
        let num = my_match( roll );
        result = format!( "{}+{}", result, num );
        counter += num;
    }
    // Formatting and returning the result.
    if number > 1 {
        format!( "{}\n{}", result, counter )
    } else {
        format!( "{}", result )
    }
}

// Function to match the specified dice type and return the corresponding result.
fn my_match( roll: &str ) -> u32 {
    match roll {
        "d2" => try_roll( 1, 2 ),
        "d3" => try_roll( 1, 3 ),
        "d4" => try_roll( 1, 4 ),
        "d5" => try_roll( 1, 5 ),
        "d6" => try_roll( 1, 6 ),
        "d8" => try_roll( 1, 8 ),
        "d10" => try_roll( 1, 10 ),
        "d12" => try_roll( 1, 12 ),
        "d20" => try_roll( 1, 20 ),
        "d100" => try_roll( 1, 100 ),
        _ => 0
    }
}

// Function to register the /roll command with necessary options.
pub fn register() -> CreateCommand {
    CreateCommand::new( "roll" ).description( "Rolling." )
        .set_options( vec![
            CreateCommandOption::new( serenity::all::CommandOptionType::String, "x-sided", "You enter the number of faces." ),
            CreateCommandOption::new( serenity::all::CommandOptionType::Integer, "counter", "Number to use x-sided." )
        ] )
}

// Function to generate a random number within a specified range.
fn try_roll( min: u32, max: u32 ) -> u32 {
    rand::thread_rng().gen_range( min..=max )
}
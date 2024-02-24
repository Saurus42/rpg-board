use rand::Rng;
use serenity::{ all::CommandDataOption, builder::{CreateCommand, CreateCommandOption} };

pub fn run( options: &[CommandDataOption] ) -> String {
    let roll = options[0].value.as_str().unwrap();
    let number = options[1].value.as_i64().unwrap();
    let num = my_match( roll );
    let mut result = num.to_string();
    let mut counter = num;
    for _n in 1..number {
        let num = my_match( roll );
        result = format!( "{}+{}", result, num );
        counter += num;
    }
    format!( "{}\n{}", result, counter )
}

fn my_match( roll: &str ) -> u32 {
    match roll {
        "d4" => try_roll( 1, 4 ),
        "d6" => try_roll( 1, 6 ),
        "d8" => try_roll( 1, 8 ),
        "d10" => try_roll( 1, 10 ),
        "d12" => try_roll( 1, 12 ),
        "d20" => try_roll( 1, 20 ),
        "d100" => try_roll( 1, 100 ),
        _ => 0
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new( "roll" ).description( "Rolling." )
        .set_options( vec![
            CreateCommandOption::new( serenity::all::CommandOptionType::String, "x-sided", "You enter the number of faces." ),
            CreateCommandOption::new( serenity::all::CommandOptionType::Integer , "number", "Number to use x-sided." )
        ] )
}

fn try_roll( min: u32, max: u32 ) -> u32 {
    rand::thread_rng().gen_range( min..=max )
}
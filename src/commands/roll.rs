use rand::Rng;
use serenity::{ all::CommandDataOption, builder::{CreateCommand, CreateCommandOption} };

pub fn run( options: &[CommandDataOption] ) -> String {
    let roll = options[0].value.as_str().unwrap();
    let number = options[1].value.as_i64().unwrap();
    let mut result = my_match( roll );
    for _n in 1..number {
        result = format!( "{}+{}", result, my_match( roll ) );
    }
    result
}

fn my_match( roll: &str ) -> String {
    match roll {
        "d4" => try_roll( 1, 4 ),
        "d6" => try_roll( 1, 6 ),
        "d8" => try_roll( 1, 8 ),
        "d10" => try_roll( 1, 10 ),
        "d12" => try_roll( 1, 12 ),
        "d20" => try_roll( 1, 20 ),
        "d100" => try_roll( 1, 100 ),
        _ => "I don't know this option.".to_owned()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new( "roll" ).description( "Rolling." )
        .set_options( vec![
            CreateCommandOption::new( serenity::all::CommandOptionType::String, "x-sided", "You enter the number of faces." ),
            CreateCommandOption::new( serenity::all::CommandOptionType::Integer , "number", "Number to use x-sided." )
        ] )
}

fn try_roll( min: u32, max: u32 ) -> String {
    rand::thread_rng().gen_range( min..=max ).to_string()
}

#[cfg(test)]
mod test {
    #[test]
    fn try_roll() {
        let result = super::try_roll( 0, 8 );
        assert_eq!( result.len(), 1 );
    }
}
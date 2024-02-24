use rand::Rng;
use serenity::{ all::CommandDataOption, builder::{CreateCommand, CreateCommandOption} };

pub fn run( options: &[CommandDataOption] ) -> String {
    println!( "{:?}", options );
    let roll = options.first().unwrap();
    let value = roll.value.as_str().unwrap();
    match value {
        "d4" => try_roll( 1, 4 ),
        "d6" => try_roll( 1, 6 ),
        "d8" => try_roll( 1, 8 ),
        "d10" => try_roll( 1, 10 ),
        "d12" => try_roll( 1, 12 ),
        "d20" => try_roll( 1, 20 ),
        "d100" => try_roll( 1, 100 ),
        _ => String::new()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new( "roll" ).description( "Rolling." )
        .set_options( vec![ CreateCommandOption::new( serenity::all::CommandOptionType::String, "x-sided", "You enter the number of faces." ) ] )
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
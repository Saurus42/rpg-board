// Importing necessary modules and traits from the standard library and external crates.
use std::io::{ Error, ErrorKind };
use serenity::{
    all::{ Interaction, Ready },
    async_trait,
    builder::{ CreateInteractionResponse, CreateInteractionResponseMessage },
    gateway::ActivityData,
    prelude::*
};
use tokio;
use dotenv::dotenv;

// Importing module containing bot commands.
mod commands;

// Asynchronous main function using tokio runtime to execute asynchronous tasks.
#[tokio::main( flavor = "current_thread" )]
async fn main() -> Result<(), Error> {
    // Loading environment variables from .env file.
    if let Err( why ) = dotenv() {
        return Err( Error::new::<String>( ErrorKind::NotFound, why.to_string() ) );
    }
    // Building the Discord bot client with necessary configurations.
    let client = Client::builder( std::env::var( "token" ).expect( "Don't have token." ), GatewayIntents::MESSAGE_CONTENT )
        .status( serenity::all::OnlineStatus::Online ).activity( ActivityData::listening( "/help" ) ).event_handler( BotHandler {} );
    // Starting the bot client and handling any errors.
    if let Ok( mut bot ) = client.await {
        if let Err( why ) = bot.start().await {
            return Err( Error::new::<String>( ErrorKind::AddrInUse, why.to_string() ) );
        }
    }
    Ok( () )
}

// Struct defining the event handler for the bot.
struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {
    // Asynchronous function to handle interactions from users.
    async fn interaction_create( &self, ctx: Context, interaction: Interaction ) {
        // Checking if the interaction is a command.
        if let Interaction::Command( command ) = interaction {
            // Matching the command name to execute the corresponding function.
            let content = match command.data.name.as_str() {
                "roll" => Some( commands::roll::run( &command.data.options ) ),
                "help" => Some( commands::help::run( &command.data.options ) ),
                _ => Some( "Not implemented.".to_owned() )
            };
            // Sending the response to the user.
            if let Some( content ) = content {
                let data = CreateInteractionResponseMessage::new().content( content );
                let builder = CreateInteractionResponse::Message( data );
                if let Err( why ) = command.create_response( &ctx.http, builder ).await {
                    println!( "{}", why.to_string() );
                }
            }
        }
    }

    // Asynchronous function called when the bot is ready to receive events.
    async fn ready( &self, ctx: Context, ready: Ready ) {
        println!( "{} is connected!", ready.user.name );
        // Fetching guilds the bot is a member of and registering commands in each guild.
        let guilds = ctx.cache.guilds();
        for guild in guilds {
            if let Err( why ) = guild.create_command( &ctx.http, commands::roll::register() ).await {
                println!( "{}", why.to_string() );
            }
            if let Err( why ) = guild.create_command( &ctx.http, commands::help::register() ).await {
                println!( "{}", why.to_string() );
            }
        }
    }
}
use std::io::{ Error, ErrorKind };
use serenity::{
    all::{ Command, Interaction, Ready },
    async_trait,
    builder::{ CreateInteractionResponse, CreateInteractionResponseMessage },
    gateway::ActivityData,
    prelude::*
};
use tokio;
use dotenv::dotenv;

mod commands;

#[tokio::main( flavor = "current_thread" )]
async fn main() -> Result<(), Error> {
    if let Err( why ) = dotenv() {
        return Err( Error::new::<String>( ErrorKind::NotFound, why.to_string() ) );
    }
    let client = Client::builder( std::env::var( "token" ).expect( "Don't have token." ), GatewayIntents::MESSAGE_CONTENT )
        .status( serenity::all::OnlineStatus::Online ).activity( ActivityData::listening( "/help" ) ).event_handler( BotHandler {} );
    if let Ok( mut bot ) = client.await {
        if let Err( why ) = bot.start().await {
            return Err( Error::new::<String>( ErrorKind::AddrInUse, why.to_string() ) );
        }
    }
    Ok( () )
}

struct BotHandler;

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create( &self, ctx: Context, interaction: Interaction ) {
        if let Interaction::Command( command ) = interaction {
            let content = match command.data.name.as_str() {
                "roll" => Some( commands::roll::run( &command.data.options ) ),
                "help" => Some( commands::help::run( &command.data.options ) ),
                _ => Some( "Not implemented.".to_owned() )
            };
            if let Some( content ) = content {
                let data = CreateInteractionResponseMessage::new().content( content );
                let builder = CreateInteractionResponse::Message( data );
                if let Err( why ) = command.create_response( &ctx.http, builder ).await {
                    println!( "{}", why.to_string() );
                }
            }
        }
    }
    async fn ready( &self, ctx: Context, ready: Ready ) {
        println!( "{} is connected!", ready.user.name );
        if let Err( why ) = Command::create_global_command( &ctx.http , commands::roll::register()).await {
            println!( "{}", why.to_string() );
        }
        if let Err( why ) = Command::create_global_command( &ctx.http , commands::help::register()).await {
            println!( "{}", why.to_string() );
        }
    }
}
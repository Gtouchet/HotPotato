mod messages;
mod client;
mod random;
mod service;
mod recoversecret;

use std::net::TcpStream;
use crate::client::Client;
use crate::messages::{Message, MessageParser};
use crate::random::Random;
use crate::service::Service;

fn main()
{
    let mut client = Client {
        service: Service {
            stream: connect_to_server("localhost:7878"),
        },
        random: Random {
            random: rand::thread_rng(),
        },
    };

    client.say_hello();

    let _ = client.subscribe(); // TODO: match

    let mut players_list : Vec<String> = Vec::new();

    loop
    {
        let message_from_server = match client.listen_to_server_message() {
            Ok(message) => message,
            Err(error) => {
                println!("Error: could not listen to server message {}", error);
                break;
            }
        };       

        match MessageParser::from_string(&message_from_server) {
            Message::EndOfGame(end_of_game) => {
                println!("END OF GAME -- {:?}", end_of_game);
                break;
            }
            Message::RoundSummary(round_summary) => {
                client.display_round_summary(round_summary);
            }
            Message::Challenge(challenge) => {
                client.handle_challenge(challenge, &players_list);
            }
            Message::PublicLeaderBoard(players) => {                
                players_list = Vec::new();

                client.display_leaderboard(&players);
                players.into_iter().for_each(|p| players_list.push(p.name));
            }
            _ => {
                println!("Error: unexpected message wtf I quit bye");
                return;
            }
        }
    }
}

fn connect_to_server(address : &str) -> TcpStream
{
    match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(err) => {
            panic!("Could not connect to the server: {}", err);
        }
    }
}

mod messages;
mod client;
mod random;
mod service;
mod recoversecret;
mod md5_resolver;

use std::net::TcpStream;
use clap::{Arg, App};
use crate::client::Client;
use crate::messages::{Message, MessageParser};
use crate::random::Random;
use crate::service::Service;

fn main()
{
    //parse CLI arguments to get server address
    let matches = App::new("HotPotato client")
        .version("v1.0.0")
        .author("the best")
        .about("HotPotato client to play against other clients")
        .arg(Arg::new("address")
            .short('a')
            .long("address")
            .value_name("ADDRESS")
            .default_value("localhost:7878")
            .help("Address of the server to connect"))
        .get_matches();

    let address = matches.value_of("address").unwrap();
    println!("Connecting to {}", &address);

    let mut client = Client {
        service: Service {
            stream: connect_to_server(address),
        },
        random: Random {
            random: rand::thread_rng(),
        },
    };

    client.say_hello();
    let response = client.subscribe();
    let client_name = response.0;
    println!("\nclient name: {}", &client_name);

    let mut players_list : Vec<String> = Vec::new();

    loop
    {
        let message_from_server = client.listen_to_response();

        match MessageParser::from_string(&message_from_server) {
            Message::EndOfGame(end_of_game) => {
                println!("\nEND OF GAME:\n ");
                client.display_leaderboard(&end_of_game.leader_board);
                break;
            }
            Message::RoundSummary(round_summary) => {
                client.display_round_summary(round_summary);
            }
            Message::Challenge(challenge) => {
                client.handle_challenge(challenge, &mut players_list, &client_name);
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
        Err(err) => panic!("Could not connect to the server: {}", err),
    }
}

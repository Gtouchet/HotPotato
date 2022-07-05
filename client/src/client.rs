use crate::{Message, Random, Service, recoversecret};
use crate::messages::{Challenge, ChallengeAnswer, ChallengeResult, PublicPlayer, MD5HashCashOutput, RoundSummary, Subscribe};
use crate::recoversecret::{*, Challenge as ChallengeTrait};

/// Client main function
///
/// # Arguments
///
/// * `service` - The service to connect and send messages to the server
/// * `random` - A random number which is used to identify the client
///
/// # Example
///
/// ```rust
/// let mut client = Client {
///         service: Service {
///         stream: connect_to_server("localhost:7878"),
///     },
///     random: Random {
///         random: rand::thread_rng(),
///     },
/// };
/// ```
pub struct Client
{
    pub service: Service,
    pub random: Random,
}

impl Client
{
    /// Say hello to the server to start the conversation
    pub(crate) fn say_hello(&mut self)
    {
        let serialized_message = serde_json::to_string(&Message::Hello).unwrap();
        self.service.send_message_and_listen_to_response(&serialized_message);
    }

    /// Subscribe to the server to get the next challenge
    ///
    /// # Return
    ///
    /// * `(String, String)` - The server response as a map
    pub(crate) fn subscribe(&mut self) -> (String, String)
    {
        let client_name = self.random.generate_name();
        let serialized_message = serde_json::to_string(&Message::Subscribe(Subscribe {
            name: client_name.clone(),
        })).unwrap();
        return (client_name, self.service.send_message_and_listen_to_response(&serialized_message));
    }

    /// Take a challenge to solve it, send the solution and a player to give the hot potato
    ///
    /// # Arguments
    ///
    /// * `challenge` - An element from Challenge enum that contains the challenge information
    /// * `players_list` - A list of game's players names
    ///
    /// # Example
    ///
    /// ```rust
    /// let message_from_server = client.listen_to_response();
    ///
    /// match MessageParser::from_string(&message_from_server) {
    ///     Message::Challenge(challenge) => {
    ///         client.handle_challenge(challenge, &players_list);
    ///     }
    /// }
    /// ```
    pub(crate) fn handle_challenge(&mut self, challenge: Challenge, players_list: &mut Vec<String>, client_name: &String)
    {
        let challenge_answer : ChallengeAnswer = match challenge {
            Challenge::RecoverSecret(input) => {
                let recover_secret: RecoverSecret = recoversecret::Challenge::new(input);
                ChallengeAnswer::RecoverSecret(recover_secret.solve())     
            }
            Challenge::MD5HashCash(_) => {
                ChallengeAnswer::MD5HashCash(MD5HashCashOutput {
                    seed: 1,
                    hashcode: "".to_string(),
                })
            }
        };
        if let Some(index) = players_list.iter().position(|value| value == client_name) {
            players_list.remove(index);
        }
        let challenge_result = ChallengeResult {
            //TODO rework this
            next_target: players_list[self.random.get_number(0, players_list.len() - 1)].to_string(),
            answer: challenge_answer
        };
    
        let serialized_message = serde_json::to_string(&Message::ChallengeResult(challenge_result)).unwrap();

        self.service.send_message(&serialized_message);
    }

    pub(crate) fn listen_to_response(&mut self) -> String
    {
        return self.service.listen_to_response();
    }

    /// Display the player leaderboard thanks to the player list
    ///
    /// # Arguments
    ///
    /// * `players_list` - A list of game's players information
    ///
    /// # Example
    ///
    /// ```rust
    /// let message_from_server = client.listen_to_response();
    ///
    /// match MessageParser::from_string(&message_from_server) {
    ///     Message::PublicLeaderBoard(players) => {
    ///         client.display_leaderboard(&players);
    ///     }
    /// }
    /// ```
    pub(crate) fn display_leaderboard(&mut self, players: &Vec<PublicPlayer>)
    {
        println!("----- Leaderboard -----\n");
        players.iter()
            .for_each(|p| println!(
                "Player {}:\n\
                - Score: {}\n\
                - Steps: {}\n\
                - Active: {}\n\
                - Used time: {}\n",
                p.name, p.score, p.steps, p.is_active, p.total_used_time)
            )
    }

    pub(crate) fn display_round_summary(&mut self, round_summary: RoundSummary)
    {
        println!("----- Round summary -----\n");
        println!("round number: {}",round_summary.chain.len().to_string());
    }
}


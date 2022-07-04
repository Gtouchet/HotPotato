use crate::{Message, Random, Service, recoversecret};
use crate::messages::{Challenge, ChallengeAnswer, ChallengeResult, PublicPlayer, MD5HashCashOutput, RoundSummary, Subscribe};
use crate::recoversecret::{*, Challenge as ChallengeTrait};

pub struct Client
{
    pub service: Service,
    pub random: Random,
}

impl Client
{
    pub(crate) fn say_hello(&mut self)
    {
        let serialized_message = serde_json::to_string(&Message::Hello).unwrap();
        self.service.send_message_and_listen_to_response(&serialized_message);
    }

    pub(crate) fn subscribe(&mut self) -> (String, String)
    {
        let client_name = self.random.generate_name();
        let serialized_message = serde_json::to_string(&Message::Subscribe(Subscribe {
            name: client_name.clone(),
        })).unwrap();
        return (client_name, self.service.send_message_and_listen_to_response(&serialized_message));
    }

    pub(crate) fn handle_challenge(&mut self, challenge: Challenge, players_list: &Vec<String>)
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
        // TODO
        println!("----- Round summary -----\n");
        //println!("{:?}", round_summary);
    }
}


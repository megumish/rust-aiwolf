use super::common::data::agent::Agent;

trait Connection<'conn> {
    fn get_connected_agent_list() -> Vec<Agent>;
    fn set_game_setting(game_setting: GameSetting);
    fn request_talk(&'conn str);
    fn request_vote(&'conn str);
    fn request_devine(&'conn str);
    fn request_attack(&'conn str);
}

mod stdio;

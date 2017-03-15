use common::data::agent::Agent;
use common::data::role::Role;
use common::net::game_setting::GameSetting;

trait Connection<'conn> {
    fn init(agent: Agent);
    fn request_name(agent: Agent) -> String;
    fn request_request_role(agent: Agent) -> Role;
    fn request_talk(agent: Agent) -> String;
    fn request_whisper(agent: Agent) -> String;
    fn request_divine_target(agent: Agent) -> Agent;
    fn request_guard_target(agent: Agent) -> Agent;
    fn request_attack_target(agent: Agent) -> Agent;
    
    fn dayStart(agent: Agent);
    fn dayFinish(agent: Agent);
    fn finish(agent: Agent);
    fn close();
}

trait Server {
    fn get_connected_agent_list() -> Vec<Agent>;
}

mod server;
mod stdio;

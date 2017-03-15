use common::net::game_setting::GameSetting;

struct IntegralServer {
    port: i32,
    limit: i32,
    agents: Vec<Agent>,
    game_setting: GameSetting,
    game_data: GameData
}

impl Server for IntegralServer {
    fn get_connected_agent_list(&self) -> Vec<Agent> {
        self.agents.clone()
    }
}

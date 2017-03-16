use common::net::game_setting::GameSetting;

struct IntegralServer {
    port: i32,
    limit: i32,
    connections: Vec<Box<Connection>>,
    game_setting: GameSetting,
    game_data: GameData
}


use std::process::Command;
use std::process::Child;
use common::data::net::game_setting::GameSetting;

struct StdIOConfiguration<'std> {
    path: &'std str,
    arg: &'std str,
}

struct StdIOConnection<S: Server> {
    process: Child,
    server: Arc<RwLock<S>>,
    agent: Agent
}

impl<'std,S: Server> StdIOConnection<S> {
    fn new_with_config(config: StdIOConfiguration<'std>, server: Arc<RwLock<S>>) -> StdIOConnection {
        let process = Command::new(config.path)
                              .arg(config.arg)
                              .spawn()
                              .expect(("failed to execute ".to_string() + config.path).as_ref());
        StdIOConnection {
            process: process,
            server: server
        }
    }
}

impl<'conn> Connection<'conn> for StdIOConnection {
    fn send(&self, packet: Packet) {
        let packet_string = Packet
        let stdin;
        match self.process.stdin {
            Some(stdin) => stdin,
            None => 
        }
        .expect(format!("Failed to open stdin of Agent[{:02}]", self.agent.get_agent_index()));
        stdin.write(packet).expect(format!("Failed to write stdin of Agent[{:02}]", self.agent.get_agent_index()));
        stdin.flush().expect(format!("Failed to flush stdin of Agent[{:02}]", self.agent.get_agent_index()));
    }
    fn init(&self) {
    }
    fn request(&self) -> String {
    }
}
//trait Connection<'conn> {
//    fn init(agent: Agent);
//    fn request_name(agent: Agent) -> String;
//    fn request_request_role(agent: Agent) -> Role;
//    fn request_talk(agent: Agent) -> String;
//    fn request_whisper(agent: Agent) -> String;
//    fn request_divine_target(agent: Agent) -> Agent;
//    fn request_guard_target(agent: Agent) -> Agent;
//    fn request_attack_target(agent: Agent) -> Agent;
//    
//    fn dayStart(agent: Agent);
//    fn dayFinish(agent: Agent);
//    fn finish(agent: Agent);
//    fn close();
//}

use std::collections::HashMap;
use std::sync::RwLock;
use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Agent {
    agent_index: i32,
}

lazy_static! {
    static ref AGENT_INDEX_MAP: RwLock<HashMap<i32, Agent>> = {
        RwLock::new(HashMap::new())
    };
}

impl Agent {

    pub fn get_agent(index: i32) -> Option<Agent> {
        if index < 0 { return None }
        match AGENT_INDEX_MAP.read().unwrap().get(&index) {
            Some(agent) => Some((*agent).clone()),
            None => {
                let agent = Agent {
                    agent_index:index 
                };
                AGENT_INDEX_MAP.write().unwrap().insert(index, agent.clone());
                Some(agent)
            }
        }
    }
    
    pub fn get_agent_index(&self) -> i32 { self.agent_index }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Agent[{:02}]", self.agent_index)
    }
}

impl Eq for Agent {}
use super::team::Team;

#[derive(Debug)]
pub enum Role {
    BODYGUARD,
    FREEMASON,
    MEDIUM,
    POSSESSED,
    SEER,
    VILLAGER,
    WEREWOLF,
    FOX,
}

impl Role {
    pub fn get_team(self) -> Team {
        match self {
            Role::BODYGUARD => Team::VILLAGER,
            Role::FREEMASON => Team::VILLAGER,
            Role::MEDIUM => Team::VILLAGER,
            Role::POSSESSED => Team::WEREWOLF,
            Role::SEER => Team::VILLAGER,
            Role::VILLAGER => Team::VILLAGER,
            Role::WEREWOLF => Team::WEREWOLF,
            Role::FOX => Team::OTHERS,
        }
    }
}
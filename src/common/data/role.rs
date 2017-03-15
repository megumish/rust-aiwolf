use super::team::Team;
use num::traits::FromPrimitive;

#[derive(Debug,Clone,Copy,PartialEq,Hash)]
pub enum Role {
    BodyGuard,
    Freemason,
    Medium,
    Possessed,
    Seer,
    Villager,
    Werewolf,
    Fox,
}

impl Role {
    pub fn get_team(self) -> Team {
        match self {
            Role::BodyGuard => Team::Villager,
            Role::Freemason => Team::Villager,
            Role::Medium => Team::Villager,
            Role::Possessed => Team::Werewolf,
            Role::Seer => Team::Villager,
            Role::Villager => Team::Villager,
            Role::Werewolf => Team::Werewolf,
            Role::Fox => Team::Others,
        }
    }
}

impl FromPrimitive for Role {
    fn from_i64(num: i64) -> Option<Role> {
        match num {
            0 => Some(Role::BodyGuard),
            1 => Some(Role::Freemason),
            2 => Some(Role::Medium),
            3 => Some(Role::Possessed),
            4 => Some(Role::Seer),
            5 => Some(Role::Villager),
            6 => Some(Role::Werewolf),
            7 => Some(Role::Fox),
            _ => None
        }
    }
    fn from_u64(num: u64) -> Option<Role> {
        match num {
            0 => Some(Role::BodyGuard),
            1 => Some(Role::Freemason),
            2 => Some(Role::Medium),
            3 => Some(Role::Possessed),
            4 => Some(Role::Seer),
            5 => Some(Role::Villager),
            6 => Some(Role::Werewolf),
            7 => Some(Role::Fox),
            _ => None
        }
    }
}

impl Eq for Role {}

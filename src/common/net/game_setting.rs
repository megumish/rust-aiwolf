use std::collections::HashMap;
use common::data::role::Role;
use num::traits::FromPrimitive;

const MIN_AGENT_NUM:usize = 3;
//Number of each roles.
//The order is as follows.
//BodyGuard, FreeMason, Medium, Possessed, Seer, Villager, Werewolf, Fox.
const ROLE_NUM_ARRAY: [[usize;8];19] = [
                                      [0,0,0,0,0,0,0,0], //0
                                      [0,0,0,0,0,0,0,0], //1
                                      [0,0,0,0,0,0,0,0], //2
                                      [0,0,0,0,1,1,1,0], //3
                                      [0,0,0,0,1,2,1,0], //4
                                      [0,0,0,1,1,2,1,0], //5
                                      [0,0,0,1,1,3,1,0], //6
                                      [0,0,0,0,1,4,2,0], //7
                                      [0,0,1,0,1,4,2,0], //8
                                      [0,0,1,0,1,5,2,0], //9
                                      [1,0,1,1,1,4,2,0], //10
                                      [1,0,1,1,1,5,2,0], //11
                                      [1,0,1,1,1,5,3,0], //12
                                      [1,0,1,1,1,6,3,0], //13
                                      [1,0,1,1,1,7,3,0], //14
                                      [1,0,1,1,1,8,3,0], //15
                                      [1,0,1,1,1,9,3,0], //16
                                      [1,0,1,1,1,10,3,0], //17
                                      [1,0,1,1,1,11,3,0]]; //18
//Number of each roles for semminers.
const SEMINAR_ARRAY: [usize;8] = [1,0,0,0,1,8,3,0];

#[derive(Clone)]
pub struct GameSetting {
    pub role_num_map: HashMap<Role, usize>,
    pub max_talk: i32,
    pub max_talk_turn: i32,
    pub max_whisper: i32,
    pub max_whisper_turn: i32,
    pub max_skip: i32,
    pub is_enable_no_attack: bool,
    pub is_vote_visible: bool,
    pub is_votable_in_first_day: bool,
    pub is_enable_no_execution: bool,
    pub is_talk_on_first_day: bool,
    pub is_validate_utterance: bool,
    pub is_whisper_before_revote: bool,
    pub time_limit: Option<i32>,
    pub max_revote: i32,
    pub max_attack_revote: i32
}

impl GameSetting {
    fn get_default_game(agent_num: usize) -> Option<GameSetting> {
        // agent_num must be bigger than MIN_AGENT_NUM
        if agent_num < MIN_AGENT_NUM { return None }
        // agent_num must be smaller than ROLE_NUM_ARRAY length
        if agent_num > ROLE_NUM_ARRAY.len() { return None }

        let mut role_num_map = HashMap::<Role, usize>::new();
        let mut num_of_role = 0;
        for role_nums in ROLE_NUM_ARRAY[agent_num].iter() {
            role_num_map.insert(Role::from_usize(num_of_role).expect("rust_aiwolf::common::data::role::Role: Out of range"), *role_nums);
            num_of_role += 1;
        }

        Some(GameSetting {
            role_num_map: role_num_map,
            max_talk: 10,
            max_talk_turn: 20,
            max_whisper: 10,
            max_whisper_turn: 20,
            max_skip: 2,
            is_enable_no_attack: false,
            is_vote_visible: true,
            is_votable_in_first_day: false,
            is_enable_no_execution: false,
            is_talk_on_first_day: false,
            is_validate_utterance: true,
            is_whisper_before_revote: false,
            time_limit: None,
            max_revote: 1,
            max_attack_revote: 1
        })
    }
}

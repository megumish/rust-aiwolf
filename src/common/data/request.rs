#[derive(Debug)]
pub enum Request {
    NAME,
    ROLE,
    TALK,
    WHISPER,
    VOTE,
    DIVINE,
    GUARD,
    ATTACK,
    INITIALIZE,
    DailyInitialize,
    DailyFinish,
    FINISH,
}

impl Request {
    pub fn has_return(self) -> bool {
        match self {
            Request::NAME => true,
            Request::ROLE => true,
            Request::TALK => true,
            Request::WHISPER => true,
            Request::VOTE => true,
            Request::DIVINE => true,
            Request::GUARD => true,
            Request::ATTACK => true,
            Request::INITIALIZE => false,
            Request::DailyInitialize => false,
            Request::DailyFinish => false,
            Request::FINISH => false
        }
    }
}
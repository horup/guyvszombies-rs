use serde::{Serialize, Deserialize};

use crate::{State, Actor, Metadata, Clock, ActorHandle, GameState};

#[derive(Serialize, Deserialize)]
pub struct ActorSnapshot {
    
}


#[derive(Serialize, Deserialize)]
pub struct StateSnapshot {
    pub spawner: Clock,
    pub me: ActorHandle,
    pub game_state: GameState,
    pub round: u32,
}

pub fn save_snapshot(state:&State, md:&Metadata) -> StateSnapshot {
    StateSnapshot {
        spawner: state.spawner.clone(),
        me: state.me.clone(),
        game_state: state.game_state.clone(),
        round: state.round.clone(),
    }
}

pub fn load_snapshot(snapshot:&StateSnapshot, md:&Metadata) -> State {
    State {
        spawner: snapshot.spawner.clone(),
        me: snapshot.me.clone(),
        actors: Default::default(),
        contact_events: Default::default(),
        round: snapshot.round.clone(),
        game_state: snapshot.game_state.clone(),
    }
}
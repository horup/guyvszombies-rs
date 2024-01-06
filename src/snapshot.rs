use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use crate::{Actor, ActorHandle, Clock, GameState, Metadata, State};

#[derive(Serialize, Deserialize)]
pub struct ActorSnapshot {
    pub handle: ActorHandle,
}

#[derive(Serialize, Deserialize)]
pub struct StateSnapshot {
    pub spawner: Clock,
    pub me: ActorHandle,
    pub game_state: GameState,
    pub round: u32,
    pub actors: Vec<ActorSnapshot>,
}

pub fn save_snapshot(state: &State, md: &Metadata) -> StateSnapshot {
    let mut actor_snapshots = Vec::default();
    for (handle, actor) in state.actors.iter() {
        actor_snapshots.push(ActorSnapshot { 
            handle: handle 
        });
    }
    StateSnapshot {
        spawner: state.spawner.clone(),
        me: state.me.clone(),
        game_state: state.game_state.clone(),
        round: state.round.clone(),
        actors: actor_snapshots,
    }
}

pub fn load_snapshot(snapshot: &StateSnapshot, md: &Metadata) -> State {
    let mut actors = SlotMap::default();
    State {
        spawner: snapshot.spawner.clone(),
        me: snapshot.me.clone(),
        actors: actors,
        contact_events: Default::default(),
        round: snapshot.round.clone(),
        game_state: snapshot.game_state.clone(),
    }
}

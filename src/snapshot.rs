//! Contains serializable data structures that captures the runtime state of the game

use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

use crate::{Actor, ActorHandle, ActorState, Clock, GameState, Metadata, State};

#[derive(Serialize, Deserialize)]
pub struct ActorSnapshot {
    pub info: String,
    pub weapon: String,
    pub state: ActorState,
}

#[derive(Serialize, Deserialize)]
pub struct StateSnapshot {
    pub spawner: Clock,
    pub me: usize,
    pub game_state: GameState,
    pub round: u32,
    pub actors: Vec<ActorSnapshot>,
}

impl StateSnapshot {
    pub fn create_snapshot(state: &State, md: &Metadata) -> StateSnapshot {
        let mut actor_snapshots = Vec::default();
        let mut me = 0;
        for (handle, actor) in state.actors.iter() {
            if state.me == handle {
                me = actor_snapshots.len();
            }
            actor_snapshots.push(ActorSnapshot {
                info: actor.info.name.clone(),
                weapon: actor.weapon.name.clone(),
                state: actor.state.clone(),
            });
        }
        StateSnapshot {
            spawner: state.spawner.clone(),
            me,
            game_state: state.game_state.clone(),
            round: state.round.clone(),
            actors: actor_snapshots,
        }
    }
    
    pub fn load_snapshot(self: &Self, md: &Metadata) -> State {
        let mut actors = SlotMap::default();
        let mut me = ActorHandle::default();
        for (index, actor) in self.actors.iter().enumerate() {
            let handle = actors.insert_with_key(|handle| Actor {
                handle,
                info: md.actors.get(&actor.info).unwrap().clone(),
                weapon: md.weapons.get(&actor.weapon).unwrap().clone(),
                state: actor.state.clone(),
            });
            if index == self.me {
                me = handle;
            }
        }
        State {
            spawner: self.spawner.clone(),
            me,
            actors: actors,
            contact_events: Default::default(),
            round: self.round.clone(),
            game_state: self.game_state.clone(),
        }
    }
}

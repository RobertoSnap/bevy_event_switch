use std::marker::PhantomData;

use bevy::{
    app::Events,
    core::FixedTimestep,
    ecs::system::{Resource, SystemParam},
    prelude::*,
};

#[derive(PartialEq, Debug, Clone, Hash)]
pub enum NetworkEvent {
    SpawnPlayer(SpawnPlayer),
    SpawnSpaceship(SpawnSpaceship),
}

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct SpawnPlayer {
    pub name: &'static str,
    pub network_id: u32,
}

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct SpawnSpaceship {
    pub damage: u32,
    pub network_id: u32,
}

const EXTERNAL_EVENT_1: NetworkEvent = NetworkEvent::SpawnPlayer(SpawnPlayer {
    name: "Player 1",
    network_id: 1,
});

const EXTERNAL_EVENT_2: NetworkEvent = NetworkEvent::SpawnSpaceship(SpawnSpaceship {
    damage: 100,
    network_id: 2,
});

pub fn add_event_switch<T>(&mut self) -> &mut Self
where
    T: Vec<Resource>,
{
    // TODO - Impl iteroator for enum (preferably, but maybe just NetworkEvent for now). Do i need some generic enum iterator with a constraint.
    self.init_resource::<Events<T>>()
        .add_system_to_stage(CoreStage::First, Events::<T>::update_system)
}

/// Sends events of type `T`.
#[derive(SystemParam)]
pub struct EventWriterSwitch<'w, 's, T: Resource> {
    // TODO - Could i store Vec of Events within a resource? Like ResMut<'w, Vec<Events<T>>>, how about initialization?
    events: ResMut<'w, Events<T>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

impl<'w, 's, T: Resource> EventWriterSwitch<'w, 's, T> {
    pub fn send(&mut self, event: T) {
        // TODO Do match here
        self.events.send(event);
    }

    // pub fn send_batch(&mut self, events: impl Iterator<Item = T>) {
    //     self.events.extend(events);
    // }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(120. / 60.))
                .with_system(parse_events),
        )
        .add_system(spawn_player)
        .add_system(spawn_spacehip);

    app.run();
}

fn parse_events(event_switch: EventWriterSwitch<NetworkEvent>) {
    event_switch.send(EXTERNAL_EVENT_1);
    event_switch.send(EXTERNAL_EVENT_2);
}

fn spawn_player(mut events: EventReader<SpawnPlayer>) {
    for event in events.iter() {
        println!("Spawning {:?}", event);
    }
}
fn spawn_spacehip(mut events: EventReader<SpawnSpaceship>) {
    for event in events.iter() {
        println!("Spawning {:?}", event);
    }
}
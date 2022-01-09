use std::marker::PhantomData;

use bevy::{
    app::Events,
    core::FixedTimestep,
    ecs::{
        event,
        system::{Resource, SystemParam},
    },
    prelude::{App, CoreStage, EventReader, ResMut, SystemSet},
    DefaultPlugins,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Debug, Clone, Hash, EnumIter)]
pub enum NetworkEvent {
    SpawnPlayer(SpawnPlayer),
    SpawnSpaceship(SpawnSpaceship),
}

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct SpawnPlayer {
    pub name: &'static str,
    pub network_id: u32,
}
impl Default for SpawnPlayer {
    fn default() -> Self {
        Self {
            network_id: Default::default(),
            name: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct SpawnSpaceship {
    pub damage: u32,
    pub network_id: u32,
}
impl Default for SpawnSpaceship {
    fn default() -> Self {
        Self {
            damage: Default::default(),
            network_id: Default::default(),
        }
    }
}

const EXTERNAL_EVENT_1: NetworkEvent = NetworkEvent::SpawnPlayer(SpawnPlayer {
    name: "Player 1",
    network_id: 1,
});

const EXTERNAL_EVENT_2: NetworkEvent = NetworkEvent::SpawnSpaceship(SpawnSpaceship {
    damage: 100,
    network_id: 2,
});

trait EventSwitch {
    fn add_event_switch<T: Resource>(&mut self) -> &mut Self;
}
impl EventSwitch for App {
    fn add_event_switch<T: Resource>(&mut self) -> &mut Self {
        {
            // For each enum variant, insert value of enum as Events<T>
            // TODO Need to accept T as Enum with some contrained for enum(T). Then iterate over it.
            self.init_resource::<Events<T>>()
                .add_system_to_stage(CoreStage::First, Events::<T>::update_system);
            // TODO - I probably need a more advanced app extend implementation, now it says update system is not in trait.
            self
        }
    }
}

// pub trait NetworkEvents2: Resource {
//     fn get_event(&self) -> Box<dyn Resource>;
// }

// impl NetworkEvents2 for NetworkEvent {
//     fn get_event(&self) -> Box<dyn Resource> {
//         match self {
//             NetworkEvent::SpawnPlayer(event) => Box::new(event),
//             NetworkEvent::SpawnSpaceship(event) => Box::new(event),
//         }
//     }
// }

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

    // TODO - Add this back again later.
    // pub fn send_batch(&mut self, events: impl Iterator<Item = T>) {
    //     self.events.extend(events);
    // }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_event_switch::<NetworkEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(120. / 60.))
                .with_system(parse_events),
        )
        .add_system(spawn_player)
        .add_system(spawn_spacehip);
    // .add_event::NetworkEvent();

    app.run();
}

fn parse_events(event_switch: EventWriterSwitch<NetworkEvent>) {
    // event_switch.send(EXTERNAL_EVENT_1);
    // event_switch.send(EXTERNAL_EVENT_2);
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

// necesary cretes
use sails_rs::{
    prelude::*,
    gstd::msg,
};

// import the state
use crate::states::event_state::{
    EventState,
    IoEventState
};

// Traffic light service struct to build the service 
#[derive(Default)]
pub struct EventService;

// Impl for seed related function to init the state
impl EventService {
    // Related function to init the service state (call only once)
    // Another related function is created that initializes the state 
    // to avoid unnecessary imports in the "lib.rs" file, you can see 
    // that it remains more "structured"
    pub fn seed() {
        EventState::init_state();
    }
}

// Trffic light service
#[service]
impl EventService {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Remote call "green" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)


    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
  

    // Remote call "yellow" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a command changing the state (&mut self)
    pub fn create(&mut self,event_end_block:u64,id_event:String)  {
        // // Get state as mut
        // let traffic_light_state = traffic_light_state_mut();
        EventState::state_mut().all_users.insert(
            msg::source(),
            id_event
        );
        EventState::state_mut().event_end_block=event_end_block;

    }


    // pub end_Event(&mut self,id_event:String)->HashMap<ActorId, String>{
    //     let mut state = EventState::state_mut();
    //     let users_for_event: HashMap<ActorId, String> = state
    //     .all_users
    //     .iter()
    //     .filter(|(_, event)| **event == id_event)
    //     .map(|(actor_id, user)| (actor_id.clone(), user.clone()))
    //     .collect();

    // // Eliminar a los usuarios que participaron en el evento
    // for (actor_id, _) in &users_for_event {
    //     state.all_users.remove(actor_id);
    // }

    // // Retorna el HashMap con los usuarios que fueron eliminados del evento
    // users_for_event


    // }

    pub fn end_event(&mut self, id_event: String) -> Vec<ActorId> {
        // Obtener el estado del evento
        let mut state = EventState::state_mut();

        // Filtrar y obtener los ActorIds asociados al evento específico
        let actor_ids_for_event: Vec<ActorId> = state
            .all_users
            .iter()
            .filter(|(_, event)| **event == id_event)
            .map(|(actor_id, _)| actor_id.clone())
            .collect();

        // Eliminar a los usuarios que participaron en el evento
        for actor_id in &actor_ids_for_event {
            state.all_users.remove(actor_id);
        }

        // Retorna el Vector con los ActorIds que fueron eliminados del evento
        actor_ids_for_event
    }


    
    // Remote call "traffic_light" exposed to external consumers
    // Returns a struct that will be sent as a response to the user
    // Is treated as a query, keeping everything unchanged and returning some data. (&self)
    pub fn events(&self) -> IoEventState {
        EventState::state_ref()
            .to_owned()
            .into()
    }
}

// // struct to use as a response to the user
// #[derive(Encode, Decode, TypeInfo)]
// #[codec(crate = sails_rs::scale_codec)]
// #[scale_info(crate = sails_rs::scale_info)]

// pub enum TrafficLightEvent {
//     Green,
//     Yellow,
//     Red
// }



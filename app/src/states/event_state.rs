// necesary cretes
use sails_rs::{
    prelude::*,
    collections::HashMap,
    // cell::Ref
};

// static mut variable (contract's state)
pub static mut EVENT_STATE: Option<EventState> = None;

// Create a struct for the state
#[derive(Clone, Default)]
pub struct EventState {
    pub event_end_block: u64,
    pub all_users: HashMap<ActorId, String>,
}

// Impl to set methods or related functions in TrafficLightState
impl EventState {
    // Method to create a new instance of TrafficLightState
    pub fn new() -> Self {
        Self {
            event_end_block:0,
            all_users:HashMap::new()
        }
    }

    // Related function to init the state of traffic light (call once)
    pub fn init_state() {
        unsafe {
            EVENT_STATE = Some(Self::new());
        };
    }

    // Related function to get the state as mut
    pub fn state_mut() -> &'static mut EventState {
        let state = unsafe { EVENT_STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Related function to get the state as ref
    pub fn state_ref() -> &'static EventState {
        let state = unsafe { EVENT_STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

// Create a struct that can be send to the user who reads state
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoEventState {
    pub event_end_block: u64,
    pub all_users: Vec<(ActorId, String)>,
}

// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<EventState> for IoEventState {

    // Conversion method
    fn from(value: EventState) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let EventState {
            event_end_block,
            all_users
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let all_users = all_users
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            event_end_block,
            all_users,
        }
    }
}
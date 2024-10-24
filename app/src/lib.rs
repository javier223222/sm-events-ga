#![no_std]

// necesary crates
use sails_rs::prelude::*;

// import our modules 
pub mod states;
pub mod services;

// Import service to be used for the program
use services::event_service::EventService;

// Traffic light program struct to build the program (there can only be one per contract)
pub struct EventProgram;

// Traffic light program, it host one or more services and it expose them to the 
// externar consumer.
// Only one program is allowed per application
#[program]
impl EventProgram
 {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        // Init the state
        EventService::seed();

        Self
    }

    // Method working with "&self", having no other parameters are treated as exposed
    // service constructors, and are called each time when an incoming request message 
    // needs be dispatched to a selected service
    // It has "message routing", This will change the way a service will be called 
    // (if omitted, the method name will be used, in this case TrafficLightSvc).
    #[route("Event")]
    pub fn traffic_light_svc(&self) -> EventService {
        EventService::new()
    }
}
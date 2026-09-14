use std::ptr;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub struct HydroServer {
    runtime: Runtime, 
    connections_count: Arc<Mutex<u32>>,
}

impl HydroServer {
    pub fn new(rt: Runtime, connections_count: Arc<Mutex<u32>>) -> HydroServer {
        HydroServer {
            runtime: rt,
            connections_count: connections_count,
        }
    }

    pub fn new_or_null() -> *mut HydroServer {
        let rt = Runtime::new().unwrap();
        let server = HydroServer::new(rt, Arc::new(Mutex::new(0)));

        return Box::into_raw(Box::new(server));
    }

    pub fn destroy(self) {
        
    }
}
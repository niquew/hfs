use std::ptr;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

pub struct HydroServer {
    runtime: Runtime, 
    connections_count: Arc<Mutex<u32>>,
}

impl HydroServer {
    fn new(rt: Runtime, connections_count: Arc<Mutex<u32>>) -> HydroServer {
        HydroServer {
            runtime: rt,
            connections_count: connections_count,
        }
    }

    fn null() -> *mut HydroServer {
        ptr::null_mut()
    }

    fn new_or_null() -> *mut HydroServer {
        let rt = Runtime::new().ok();
        if let Some(runtime) = rt {
            let server = HydroServer::new(runtime, Arc::new(Mutex::new(0)));

            return Box::into_raw(Box::new(server));
        }
        HydroServer::null()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn thfs_create_server() -> *mut HydroServer {
    HydroServer::new_or_null()
}
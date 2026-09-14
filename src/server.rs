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

    fn new_or_null() -> *mut HydroServer {
        let rt = Runtime::new().unwrap();
        let server = HydroServer::new(rt, Arc::new(Mutex::new(0)));

        return Box::into_raw(Box::new(server));
    }

    fn destroy(self) {
        
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hfs_create_server() -> *mut HydroServer {
    HydroServer::new_or_null()
}

pub extern "C" fn hfs_destroy_server(server: *mut HydroServer) {
    if !server.is_null()
    {
        unsafe {
            let boxed_server = Box::from_raw(server);
            boxed_server.destroy();
        }
    }   
}
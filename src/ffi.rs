use crate::server::HydroServer;

#[unsafe(no_mangle)]
pub extern "C" fn hfs_create_server() -> *mut HydroServer {
    HydroServer::new_or_null()
}

#[unsafe(no_mangle)]
pub extern "C" fn hfs_destroy_server(server: *mut HydroServer) {
    if !server.is_null()
    {
        unsafe {
            let boxed_server = Box::from_raw(server);
            boxed_server.destroy();
        }
    }   
}
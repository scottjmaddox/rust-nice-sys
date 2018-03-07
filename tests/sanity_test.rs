extern crate nice_sys;

use std::os::raw::c_void;
use std::ptr;
use nice_sys::*;

#[test]
fn sanity_test() {
    unsafe {
        let gctx = ptr::null_mut();
        let gloop = g_main_loop_new(gctx, 0);
        assert!(!gloop.is_null());
        let agent = nice_agent_new(
            g_main_loop_get_context(gloop),
            NiceCompatibility_NICE_COMPATIBILITY_RFC5245,
        );
        assert!(!agent.is_null());
        g_object_unref(agent as *mut c_void);
        g_main_loop_unref(gloop);
    }
}

use std::{
    path::Path,
};
use ocl_include::{MemHook};

include!(concat!(env!("OUT_DIR"), "/ocl_src_list.rs"));

/// OpenCL source code tree.
pub fn source() -> MemHook {
    let mut hook = MemHook::new();
    let pref = Path::new("clay_core");
    for (name, content) in OCL_SRC_LIST.iter() {
        hook.add_file(
            &pref.join(name),
            content.to_string(),
        ).unwrap();
    }
    hook
}

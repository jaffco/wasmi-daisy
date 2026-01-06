#![no_std]

extern crate alloc;

use core::slice;
use wasmi::*;

// Use Jaffx SDRAM allocator via extern C callbacks
use core::alloc::{GlobalAlloc, Layout};

// External C functions provided by Jaffx SDRAM manager
extern "C" {
    fn jaffx_sdram_malloc(size: usize) -> *mut u8;
    fn jaffx_sdram_free(ptr: *mut u8);
}

struct JaffxSdramAllocator;

unsafe impl Sync for JaffxSdramAllocator {}

unsafe impl GlobalAlloc for JaffxSdramAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        jaffx_sdram_malloc(layout.size())
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        jaffx_sdram_free(ptr);
    }
}

#[global_allocator]
static ALLOCATOR: JaffxSdramAllocator = JaffxSdramAllocator;

/// Opaque handle to a wasmi engine
#[repr(C)]
pub struct WasmiEngine {
    _private: [u8; 0],
}

/// Opaque handle to a wasmi store
#[repr(C)]
pub struct WasmiStore {
    _private: [u8; 0],
}

/// Opaque handle to a wasmi module
#[repr(C)]
pub struct WasmiModule {
    _private: [u8; 0],
}

/// Opaque handle to a wasmi instance
#[repr(C)]
pub struct WasmiInstance {
    _private: [u8; 0],
}

/// Opaque handle to a wasmi function
#[repr(C)]
pub struct WasmiFunc {
    _private: [u8; 0],
}

/// Create a new wasmi engine
/// Returns null on failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_engine_new() -> *mut WasmiEngine {
    let engine = Engine::default();
    let boxed = alloc::boxed::Box::new(engine);
    alloc::boxed::Box::into_raw(boxed) as *mut WasmiEngine
}

/// Delete a wasmi engine
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_engine_delete(engine: *mut WasmiEngine) {
    if !engine.is_null() {
        let _ = alloc::boxed::Box::from_raw(engine as *mut Engine);
    }
}

/// Create a new wasmi store
/// Returns null on failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_store_new(engine: *const WasmiEngine) -> *mut WasmiStore {
    if engine.is_null() {
        return core::ptr::null_mut();
    }
    
    let engine_ref = &*(engine as *const Engine);
    let store = Store::new(engine_ref, ());
    let boxed = alloc::boxed::Box::new(store);
    alloc::boxed::Box::into_raw(boxed) as *mut WasmiStore
}

/// Delete a wasmi store
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_store_delete(store: *mut WasmiStore) {
    if !store.is_null() {
        let _ = alloc::boxed::Box::from_raw(store as *mut Store<()>);
    }
}

/// Load a WebAssembly module from bytes
/// Returns null on failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_module_new(
    engine: *const WasmiEngine,
    wasm_bytes: *const u8,
    wasm_len: usize,
) -> *mut WasmiModule {
    if engine.is_null() || wasm_bytes.is_null() {
        return core::ptr::null_mut();
    }
    
    let engine_ref = &*(engine as *const Engine);
    let bytes = slice::from_raw_parts(wasm_bytes, wasm_len);
    
    match Module::new(engine_ref, bytes) {
        Ok(module) => {
            let boxed = alloc::boxed::Box::new(module);
            alloc::boxed::Box::into_raw(boxed) as *mut WasmiModule
        }
        Err(_) => core::ptr::null_mut(),
    }
}

/// Delete a wasmi module
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_module_delete(module: *mut WasmiModule) {
    if !module.is_null() {
        let _ = alloc::boxed::Box::from_raw(module as *mut Module);
    }
}

/// Instantiate a WebAssembly module
/// Returns null on failure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_instance_new(
    store: *mut WasmiStore,
    module: *const WasmiModule,
) -> *mut WasmiInstance {
    if store.is_null() || module.is_null() {
        return core::ptr::null_mut();
    }
    
    let store_ref = &mut *(store as *mut Store<()>);
    let module_ref = &*(module as *const Module);
    
    let linker: Linker<()> = Linker::new(store_ref.engine());
    
    match linker.instantiate_and_start(store_ref, module_ref) {
        Ok(instance) => {
            let boxed = alloc::boxed::Box::new(instance);
            alloc::boxed::Box::into_raw(boxed) as *mut WasmiInstance
        }
        Err(_) => core::ptr::null_mut(),
    }
}

/// Delete a wasmi instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_instance_delete(instance: *mut WasmiInstance) {
    if !instance.is_null() {
        let _ = alloc::boxed::Box::from_raw(instance as *mut Instance);
    }
}

/// Get an exported function by name
/// Returns null if not found or not a function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_instance_get_func(
    store: *mut WasmiStore,
    instance: *const WasmiInstance,
    name: *const u8,
    name_len: usize,
) -> *mut WasmiFunc {
    if store.is_null() || instance.is_null() || name.is_null() {
        return core::ptr::null_mut();
    }
    
    let store_ref = &mut *(store as *mut Store<()>);
    let instance_ref = &*(instance as *const Instance);
    let name_bytes = slice::from_raw_parts(name, name_len);
    let name_str = match core::str::from_utf8(name_bytes) {
        Ok(s) => s,
        Err(_) => return core::ptr::null_mut(),
    };
    
    match instance_ref.get_export(store_ref, name_str) {
        Some(Extern::Func(func)) => {
            let boxed = alloc::boxed::Box::new(func);
            alloc::boxed::Box::into_raw(boxed) as *mut WasmiFunc
        }
        _ => core::ptr::null_mut(),
    }
}

/// Call a function that takes two i32 params and returns one i32
/// Returns 0 on error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_func_call_i32_i32_to_i32(
    store: *mut WasmiStore,
    func: *const WasmiFunc,
    arg0: i32,
    arg1: i32,
) -> i32 {
    if store.is_null() || func.is_null() {
        return 0;
    }
    
    let store_ref = &mut *(store as *mut Store<()>);
    let func_ref = &*(func as *const Func);
    
    let mut results = [Val::I32(0)];
    match func_ref.call(store_ref, &[Val::I32(arg0), Val::I32(arg1)], &mut results) {
        Ok(_) => {
            if let Val::I32(result) = results[0] {
                result
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Call a function that takes one f32 param and returns one f32
/// Returns 0.0 on error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_func_call_f32_to_f32(
    store: *mut WasmiStore,
    func: *const WasmiFunc,
    arg: f32,
) -> f32 {
    if store.is_null() || func.is_null() {
        return 0.0;
    }
    
    let store_ref = &mut *(store as *mut Store<()>);
    let func_ref = &*(func as *const Func);
    
    let mut results = [Val::F32(0.0.into())];
    match func_ref.call(store_ref, &[Val::F32(arg.into())], &mut results) {
        Ok(_) => {
            if let Val::F32(result) = results[0] {
                result.into()
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    }
}

/// Call a function that processes float buffers
/// Copies input_buffer to WASM memory, calls function, copies output back
/// WASM function signature: (i32 input_ptr, i32 output_ptr, i32 size) -> void
/// Returns 0 on success, -1 on error
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_func_call_buffer_process(
    store: *mut WasmiStore,
    instance: *mut WasmiInstance,
    func: *const WasmiFunc,
    input_buffer: *const f32,
    output_buffer: *mut f32,
    buffer_size: usize,
) -> i32 {
    if store.is_null() || instance.is_null() || func.is_null() || input_buffer.is_null() || output_buffer.is_null() {
        return -1;
    }
    
    let store_ref = &mut *(store as *mut Store<()>);
    let instance_ref = &*(instance as *const Instance);
    let func_ref = &*(func as *const Func);
    
    // Get WASM memory
    let memory = match instance_ref.get_export(&mut *store_ref, "memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => return -1,
    };
    
    let byte_size = buffer_size * 4; // f32 = 4 bytes
    
    // Use fixed memory locations: input at offset 0, output at offset after input
    let input_offset = 0;
    let output_offset = byte_size;
    
    // Copy input buffer to WASM memory
    let input_slice = slice::from_raw_parts(input_buffer as *const u8, byte_size);
    if memory.write(&mut *store_ref, input_offset, input_slice).is_err() {
        return -1;
    }
    
    // Call WASM function with memory offsets
    let args = [
        Val::I32(input_offset as i32),
        Val::I32(output_offset as i32),
        Val::I32(buffer_size as i32),
    ];
    
    if func_ref.call(&mut *store_ref, &args, &mut []).is_err() {
        return -1;
    }
    
    // Copy output buffer from WASM memory
    let output_slice = slice::from_raw_parts_mut(output_buffer as *mut u8, byte_size);
    if memory.read(&mut *store_ref, output_offset, output_slice).is_err() {
        return -1;
    }
    
    0
}

/// Delete a function handle
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmi_func_delete(func: *mut WasmiFunc) {
    if !func.is_null() {
        let _ = alloc::boxed::Box::from_raw(func as *mut Func);
    }
}

// Panic handler for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

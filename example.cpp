/**
 * Example: Using wasmi-daisy in C++ for Daisy Seed
 * 
 * This demonstrates how to embed WebAssembly modules in your Daisy project
 */

#include "wasmi_daisy.h"

// Example wasm bytecode (adds two numbers)
// (module
//   (func (export "add") (param i32 i32) (result i32)
//     local.get 0
//     local.get 1
//     i32.add))
static const uint8_t wasm_add[] = {
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01,
    0x7f, 0x03, 0x02, 0x01, 0x00, 0x07, 0x07, 0x01,
    0x03, 0x61, 0x64, 0x64, 0x00, 0x00, 0x0a, 0x09,
    0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a,
    0x0b
};

void example_usage() {
    // Create engine
    WasmiEngine* engine = wasmi_engine_new();
    if (!engine) return;
    
    // Create store
    WasmiStore* store = wasmi_store_new(engine);
    if (!store) {
        wasmi_engine_delete(engine);
        return;
    }
    
    // Load module
    WasmiModule* module = wasmi_module_new(engine, wasm_add, sizeof(wasm_add));
    if (!module) {
        wasmi_store_delete(store);
        wasmi_engine_delete(engine);
        return;
    }
    
    // Instantiate module
    WasmiInstance* instance = wasmi_instance_new(store, module);
    if (!instance) {
        wasmi_module_delete(module);
        wasmi_store_delete(store);
        wasmi_engine_delete(engine);
        return;
    }
    
    // Get exported function
    const char* func_name = "add";
    WasmiFunc* add_func = wasmi_instance_get_func(
        store, 
        instance,
        reinterpret_cast<const uint8_t*>(func_name),
        3  // strlen("add")
    );
    
    if (add_func) {
        // Call the function: 5 + 3 = 8
        int32_t result = wasmi_func_call_i32_i32_to_i32(store, add_func, 5, 3);
        // result should be 8
        
        wasmi_func_delete(add_func);
    }
    
    // Cleanup
    wasmi_instance_delete(instance);
    wasmi_module_delete(module);
    wasmi_store_delete(store);
    wasmi_engine_delete(engine);
}

# wasmi-daisy

WebAssembly interpreter for Daisy Seed - A minimal no_std wrapper around wasmi for embedded ARM Cortex-M7F targets.

## What This Is

This library provides a C-compatible FFI interface to the `wasmi` WebAssembly interpreter, compiled specifically for the Daisy Seed platform (ARM Cortex-M7F, thumbv7em-none-eabihf).

Unlike wasmi's official C API (which requires `std`), this wrapper:
- ✅ Compiles for bare-metal ARM with `no_std`
- ✅ Includes a simple bump allocator (512KB heap)
- ✅ Provides C-compatible functions callable from C++
- ✅ Uses wasmi natively (proven to work on ARM)

## Build

```bash
./build.sh
```

This creates:
- `target/thumbv7em-none-eabihf/release/libwasmi_daisy.a` - Static library
- `wasmi_daisy.h` - C header file

## Integration with Daisy Project

### 1. Add to your Makefile

```make
# Add include path
C_INCLUDES += -I/path/to/wasmi-daisy

# Add library to linker flags
LDFLAGS += /path/to/wasmi-daisy/target/thumbv7em-none-eabihf/release/libwasmi_daisy.a
```

### 2. Include the header in your C++ code

```cpp
#include "wasmi_daisy.h"
```

### 3. Use the API

See `example.cpp` for a complete usage example.

## API Overview

### Engine Management
- `WasmiEngine* wasmi_engine_new()` - Create engine
- `void wasmi_engine_delete(WasmiEngine*)` - Delete engine

### Store Management
- `WasmiStore* wasmi_store_new(const WasmiEngine*)` - Create store
- `void wasmi_store_delete(WasmiStore*)` - Delete store

### Module Management
- `WasmiModule* wasmi_module_new(engine, bytes, len)` - Load wasm module
- `void wasmi_module_delete(WasmiModule*)` - Delete module

### Instance Management
- `WasmiInstance* wasmi_instance_new(store, module)` - Instantiate module
- `void wasmi_instance_delete(WasmiInstance*)` - Delete instance

### Function Calling
- `WasmiFunc* wasmi_instance_get_func(store, instance, name, name_len)` - Get exported function
- `int32_t wasmi_func_call_i32_i32_to_i32(store, func, arg0, arg1)` - Call function
- `void wasmi_func_delete(WasmiFunc*)` - Delete function handle

## Memory Configuration

The library uses the **Jaffx SDRAM allocator** for all memory allocations. This gives you:

- ✅ Proper memory management (malloc/free with coalescing)
- ✅ Full 64MB SDRAM available for WebAssembly modules
- ✅ No fixed heap size - grows as needed
- ✅ Memory can be freed and reused

### Required Integration

You **must** provide these extern "C" functions in your application:

```cpp
extern "C" {
  void* jaffx_sdram_malloc(size_t size) {
    return Jaffx::mSDRAM.malloc(size);
  }
  
  void jaffx_sdram_free(void* ptr) {
    Jaffx::mSDRAM.free(ptr);
  }
}
```

These delegate to the Jaffx SDRAM manager which handles all allocation bookkeeping.

## Build Details

- **Target**: thumbv7em-none-eabihf (ARM Cortex-M7F with hardware FPU)
- **Optimization**: Size-optimized (`opt-level = "z"`)
- **LTO**: Enabled for smaller binary size
- **Dependencies**: wasmi (no_std mode)

## Technical Notes

### Why This Works When Official C API Doesn't

The official wasmi C API (`wasmi_c_api`) fails to build for ARM because:
1. It builds as `cdylib`/`staticlib` crate type
2. This triggers a bug in the `spin` v0.9.8 dependency
3. The bug causes missing Rust prelude imports for certain ARM targets

This wrapper solves the problem by:
1. Using wasmi directly as a Rust library dependency
2. Building as a `staticlib` without the intermediate `wasmi_c_api` layer
3. Providing our own FFI bindings

### Allocator Strategy

The Jaffx SDRAM allocator provides:
- ✅ Dynamic allocation and deallocation
- ✅ Automatic coalescing of free blocks
- ✅ Efficient memory reuse
- ✅ Full 64MB SDRAM address space
- ✅ Proper bookkeeping with metadata

No configuration needed - the Jaffx SDRAM manager handles everything!

## License

This wrapper follows the same license as wasmi (MIT/Apache-2.0).

## Size Estimate

The compiled `libwasmi_daisy.a` is approximately 1-2MB. This includes:
- wasmi interpreter
- WebAssembly parser
- Our FFI wrapper
- Allocator implementation

Use `arm-none-eabi-size` on your final binary to see actual memory usage.

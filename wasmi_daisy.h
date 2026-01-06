/* Wasmi C API for Daisy Seed
 * Minimal no_std wrapper around wasmi for embedded ARM targets
 * 
 * IMPORTANT: You must provide these allocation functions from C++:
 *   void* jaffx_sdram_malloc(size_t size);
 *   void jaffx_sdram_free(void* ptr);
 * 
 * These should delegate to your Jaffx::mSDRAM allocator.
 */

#ifndef WASMI_DAISY_H
#define WASMI_DAISY_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Allocation functions that must be provided by the application */
void* jaffx_sdram_malloc(size_t size);
void jaffx_sdram_free(void* ptr);

/* Opaque handles */
typedef struct WasmiEngine WasmiEngine;
typedef struct WasmiStore WasmiStore;
typedef struct WasmiModule WasmiModule;
typedef struct WasmiInstance WasmiInstance;
typedef struct WasmiFunc WasmiFunc;

/* Engine management */
WasmiEngine* wasmi_engine_new(void);
void wasmi_engine_delete(WasmiEngine* engine);

/* Store management */
WasmiStore* wasmi_store_new(const WasmiEngine* engine);
void wasmi_store_delete(WasmiStore* store);

/* Module management */
WasmiModule* wasmi_module_new(const WasmiEngine* engine, const uint8_t* wasm_bytes, size_t wasm_len);
void wasmi_module_delete(WasmiModule* module);

/* Instance management */
WasmiInstance* wasmi_instance_new(WasmiStore* store, const WasmiModule* module);
void wasmi_instance_delete(WasmiInstance* instance);

/* Function management */
WasmiFunc* wasmi_instance_get_func(WasmiStore* store, const WasmiInstance* instance, const uint8_t* name, size_t name_len);
void wasmi_func_delete(WasmiFunc* func);

/* Function calling - simplified for common cases */
int32_t wasmi_func_call_i32_i32_to_i32(WasmiStore* store, const WasmiFunc* func, int32_t arg0, int32_t arg1);
float wasmi_func_call_f32_to_f32(WasmiStore* store, const WasmiFunc* func, float arg);

/* Buffer processing - for audio/DSP
 * Copies input_buffer to WASM memory, calls function with (input_ptr, output_ptr, size),
 * then copies WASM memory back to output_buffer.
 * WASM function signature must be: (i32, i32, i32) -> void
 * Returns 0 on success, -1 on error
 */
int32_t wasmi_func_call_buffer_process(
    WasmiStore* store,
    WasmiInstance* instance,
    const WasmiFunc* func,
    const float* input_buffer,
    float* output_buffer,
    size_t buffer_size
);

#ifdef __cplusplus
}
#endif

#endif /* WASMI_DAISY_H */

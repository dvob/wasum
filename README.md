# wasum
wasum (WASM Summary) prints imported and exported types of a WASM module.

```
wasum my_module.wasm
```

Example output:
```
exports:
memory 'memory' MemoryType { ty: Memory { minimum: 17, maximum: None, shared: false, memory64: false } }
global __heap_base i32 (mutable)
global __data_end i32 (mutable)
func _start()
func main(i32, i32) -> i32

imports:
func wasi_snapshot_preview1 fd_read(i32, i32, i32, i32) -> i32
func wasi_snapshot_preview1 fd_write(i32, i32, i32, i32) -> i32
func wasi_snapshot_preview1 environ_get(i32, i32) -> i32
func wasi_snapshot_preview1 environ_sizes_get(i32, i32) -> i32
func wasi_snapshot_preview1 proc_exit(i32)
```
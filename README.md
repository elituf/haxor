## haxor
windows external memory hacking library

## using
```rust
let proc = Process::find("notepad.exe")?;
let some_val = proc.read_mem::<i32>(0xDEADBEEF)?;
```

```rust
let proc = Process::find(1337)?;
let chain: Vec<usize> = vec![proc.base_address, 0x4B1D, 0x8, 0x12];
let some_addr: usize = proc.resolve_pointer_chain(&chain)?;
let some_val = proc.read_mem::<u8>(some_addr)?;
```

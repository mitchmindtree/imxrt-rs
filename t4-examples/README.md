# Download examples to a Teensy 4

Make sure you have all the [dependencies](https://github.com/mciantyre/teensy4-rs#dependencies) specified in the `teensy4-rs` project. Let `teensy_loader_cli` be the command-line loader tool for the Teensy boards. 

From this directory,

```
cargo objcopy --bin rtic --release -- -O ihex rtic.hex
teensy_loader_cli -v -w --mcu=TEENSY40 rtic.hex
```

Success: the Teensy's LED turns on.
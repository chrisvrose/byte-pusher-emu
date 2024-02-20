# BytePusher Emulator

This is a play at emulating a BytePusher machine developed by [Javamannen](https://esolangs.org/wiki/User:Javamannen).
Binary builds for linux x86_64 is available in GitHub Action runs.

![Screen test](assets/screen_test.png)
Screenshot program author: [Javamannen](https://esolangs.org/wiki/User:Javamannen)

## Status

- [X] Memory
- [X] Color
- [X] CPU
  - [X] Inner loop - 65536 instructions
  - [X] Outer loop - 60fps display loop
- [X] Display adapter
  - [X] SDL2 adapter
- [X] Keyboard
- [X] Audio
- [X] Load a ROM

## Usage instructions

For help on how to use, please refer
```sh
./byte-pusher-emu --help
```

To load a program, use the following command.

```sh
./byte-pusher-emu -f rom.BytePusher
```

Loads a rom from the file specified. If `-f` is not mentioned, the VM will run with an empty memory (which will cause it to cycle endlessly).

## More information about the BytePusher VM

https://esolangs.org/wiki/BytePusher
# BytePusher Emulator

`byte-pusher-emu` is an emulator for the BytePusher virtual machine developed by [Javamannen](https://esolangs.org/wiki/User:Javamannen).

Binary builds for linux x86_64 is available in the Releases section.


![Audio test](assets/audio_test.png)

<details>
<summary>More screenshots</summary>

#### Palette Test
![Screen test](assets/screen_test.png)

#### Keyboard test
![Keyboard test](assets/kb_test.png)

</details>

The roms showcased here have been created by [Javamannen](https://esolangs.org/wiki/User:Javamannen).

Note: This is a project that was used to learn Rust and emulation basics. However, it should work good as a reference for running ROMs.

## Usage instructions

`byte-pusher-emu` is rather minimalistic, and needs to be launched from CLI.

For help on how to use, run the following:
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

## Development Status

- [X] Memory
- [X] Color
- [X] CPU
- [X] Display adapter - SDL2
- [X] Keyboard - SDL2
- [X] Audio - SDL2
- [X] Load a ROM
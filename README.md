# Klocka

## Browser support

Currently only Chrome is supported. Firefox has support for Push, but requires a
custom encryption scheme that I haven't got around to, yet. No other browsers
currently implement the Push API required for this to function.

## Customizing

If you don't want to make it bind any GPIO pins, set the environment variable
`KLOCKA_TRIGGER` to `FD`. If you want to change the pin it uses (22 by default),
change `GPIO_PIN` in `server/src/trigger/mod.rs`.

## Building (client)

In `web-client/manifest.json`, change `gcm_sender_id` to your Google project ID.

## Building (server)

Create a file called `server/gcm_key.txt` containing **ONLY** your Google Cloud
Messaging key. Afterwards you should be able to `cargo run` (from the `server`
directory).

### Cross-compiling for the RPi 2/3

You'll need:

* A `rustc` target (install with `rustup target add armv7-unknown-linux-gnueabihf`)
* A GCC for linking to the target platform (on Arch, this means `armv7-unknown-linux-gnueabihf` from the AUR)
* OpenSSL for the target platform (the easiest way is to copy `openssl-build-arm.sh` to the OpenSSL source directory, adapt the paths inside it, and run it)

Afterwards, you'll need to set up your `~/.cargo/config` so it knows about your
GCC install, and tell `server/crossbuild-arm.sh` about your OpenSSL install
directory. `~/.cargo/config` should contain at least the following:

```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Finally, simply run `server/crossbuild-arm.sh` to compile!

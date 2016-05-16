# Klocka

## [Demo](https://www.youtube.com/watch?v=GejDrrDToao)

## Motivation

Well, I recently got [an awful (cheap) doorbell](http://www.clasohlson.com/se/Tr%C3%A5dl%C3%B6s-d%C3%B6rrklocka/36-6016) from Clas Ohlson. As it turns out, my AA batteries wouldn't even fit inside the receiver, and the speaker sounds awful. But, what if I could hook up the receiver to a RPi instead? Hmm...

## Usage

When compiled, this should create a single self-contained binary, which listens
on port 8080 for the web interface and binds to the GPIO pin 22. To register
your browser, go to http://localhost:8080/ and approve the permission request.
If you want to be able to register a remote device, you'll need to put a SSL
terminating reverse proxy in front of this, since browsers for some reason don't
support push notifications from unencrypted websites.

To make things worse, at least Chrome doesn't like loading workers from untrusted
domains, so even a self-signed certificate won't work out of the box. To get
around this, you'll need to download the certificate to your browser and
whitelist it. Alternately, sign it through a service like Let's Encrypt, but that
requires you to expose the service to the public, which probably isn't a great idea...

## Limitations

At least on Chrome and Firefox for desktop, the ringtone doesn't trigger unless
the page is open. This should change when/if browsers start supporting the `audio`
parameter for notifications...

## Browser support

Currently only Google Chrome and Firefox are supported. No other browsers
currently implement the Push API required for this to function.

## Customizing

If you don't want to make it bind any GPIO pins, set the environment variable
`KLOCKA_TRIGGER` to `FD`, which will make it trigger on newlines in stdin instead. If you want to change the pin it uses (BCM 22 by default),
change `GPIO_PIN` in `server/src/trigger/mod.rs`. The port for the web UI is set as `WEB_PORT` in `server/src/main.rs`.

## Building (physically)

Use a multimeter to find out which cable to the LED carries the signal (you
might want to take a picture). Desolder the cables to the battery enclosure,
speaker, and LED. Solder jumper cables to where both cables to the battery
enclosure went, as well as for the signal that went to the LED. Connect the two
power cables to a 3v3 pin and a ground pin on the Pi (the one that went to the
positive battery pole should go to the 3v3 pin), and the LED cable to BCM pin
22 (physical pin 15).

## Building (client)

In `web-client/manifest.json`, change `gcm_sender_id` to your Google project ID.

## Building (server)

Create a file called `server/gcm_key.txt` containing **ONLY** your Google Cloud
Messaging key. Afterwards you should be able to `cargo run` (from the `server`
directory).

### Cross-compiling for the RPi 2/3

You'll need:

* A `rustc` target (install with `rustup target add armv7-unknown-linux-gnueabihf`)
* A GCC for the target platform (on Arch, this means `arm-unknown-linux-gnueabihf` from the AUR)

(Note: Currently the AUR's ARM version of GCC is broken, you'll need to add `--std=gnu++03` to `$CXXFLAGS` in the `PKGBUILD` for GCC 6 to want to build GCC 5)

First, sync git submodules and run `server/openssl-build-arm.sh` to build OpenSSL. Then, run `server/crossbuild-arm.sh` to compile!

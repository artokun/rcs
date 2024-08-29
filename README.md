This is a game about programming reaction control systems for space ships.

# Gameplay
# UI Readouts
- Frame Shift
    - MULT - Multiplier (2x, 4x, 8x)
- Proximity
    - RCS - Relative Collision Speed
    - DANGER - Danger level
- Map
    - Self
    - Target
- Readouts
    - Target
        - Id
        - Name
        - Class
        - Model
        - Diameter

        - VREL - Velocity relative to target
        - VCRS - Cross-track velocity relative to target
        - RNG - Range to target
        - BRG - Bearing to target
        - ETA - Estimated time of arrival
    - Delta-V
        - SPD - Speed
        - FUEL - Fuel remaining in kg
        - PWR - Power remaining in kWh
  
# Game background
This game takes place far into the future where humans have colonized other planets and established space-faring empires. Unfortunately there was a massive AI virus that took place and shut down all of the AI networks.

You are a ship navigation programmer, and your job is to create routines for various tasks ships need to perform in this new post-AI world. You are responsible for thrust vector control, and attitude control. You can also dock with various space stations to transfer goods, fuel and personnel.

You can interface with the ship's systems via a small console in the cockpit. Here you can create and test your routines. But watch out! If you make a mistake, you can end up with a crippled ship or even a crashed ship.

The first ship you are entrusted with is a small cargo tug, that is designed to transfer cargo between a space station and a transport ship. It is small and can only carry one SCU (Standard Cargo Unit) at a time. It has limited reaction mass, and low thrust. There's also a ton of them, so you can just wait for one to become available. You don't pilot the tugs the same way you do a fighter or a transport ship. Instead you can program their behavior remotely from a central control station where they are operating out of.

You must create a routine that will approach the space station, and dock with it. Accept one SCU from the station, and then move to the transport ship and deposit the SCU. Once the SCU is deposited, move back to the space station, and repeat the process. The space station has infinite SCUs to offer. The transport ship can hold 100 SCUs. Once the transport ship is full, it will depart the space station.

The space station has a small refueling section that can transfer fuel from the station to the cargo tug under your control.

The cargo tug has a small cargo hold that can hold one SCU at a time.

The cargo tug has a small reaction mass tank that can hold 100 units of reaction mass.

## To release on web

```bash
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./dist/out/ --target web ./target/wasm32-unknown-unknown/release/rcs.wasm
```
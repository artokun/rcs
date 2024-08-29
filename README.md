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


## Notes

### VREL and VCRL
```rs
use bevy::prelude::*;

#[derive(Component)]
struct Spaceship {
    velocity: Vec2,
}

#[derive(Component)]
struct Target {
    velocity: Vec2,
}

fn calculate_velocities(
    spaceship_query: Query<(&Transform, &Spaceship)>,
    target_query: Query<(&Transform, &Target)>,
) {
    let (spaceship_transform, spaceship) = spaceship_query.single();
    let (target_transform, target) = target_query.single();

    let spaceship_position = spaceship_transform.translation.truncate();
    let target_position = target_transform.translation.truncate();

    // Calculate VREL
    let v_rel = spaceship.velocity - target.velocity;

    // Calculate vector from target to spaceship
    let r = spaceship_position - target_position;
    let r_hat = r.normalize();

    // Calculate VCRL
    let v_crl = v_rel - v_rel.dot(r_hat) * r_hat;

    println!("VREL: {:?}", v_rel);
    println!("VCRL: {:?}", v_crl);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, calculate_velocities)
        .run();
}
```

### RCS Particle System
```rs
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::prelude::*;

// Component to mark entities as RCS thrusters
#[derive(Component)]
struct RCSThruster {
    active: bool,
}

// Component for particles
#[derive(Component)]
struct Particle {
    lifetime: Timer,
    velocity: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_thrusters, update_particles))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the RCS thruster
    commands.spawn((
        RCSThruster { active: false },
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}

fn update_thrusters(
    mut commands: Commands,
    mut thruster_query: Query<(&mut RCSThruster, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut thruster, transform) in thruster_query.iter_mut() {
        // Toggle thruster with spacebar
        if keyboard_input.just_pressed(KeyCode::Space) {
            thruster.active = !thruster.active;
        }

        if thruster.active {
            // Spawn particles
            if random::<f32>() < 0.5 {
                let mut rng = rand::thread_rng();
                let angle = rng.gen_range(-15.0..15.0f32).to_radians();
                let speed = rng.gen_range(50.0..100.0);
                let velocity = Vec2::new(angle.sin() * speed, angle.cos() * speed);

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(2.0).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                    Particle {
                        lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                        velocity,
                    },
                ));
            }
        }
    }
}

fn update_particles(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Transform, &mut Particle)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            let t = 1.0 - particle.lifetime.percent_left();
            transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
            transform.scale = Vec3::splat(1.0 - t);
        }
    }
}
```
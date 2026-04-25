use gilrs::{Gilrs, Button, Axis, Event, EventType};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize GilRs with the XInput backend (based on your Cargo.toml)
    let mut gilrs = Gilrs::new().unwrap();
    println!("Xbox Controller Active. Move sticks or press A/B/X/Y...");

    loop {
        // 1. Process Events (Buttons)
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::South => println!("Button: A"),
                        Button::East => println!("Button: B"),
                        Button::West => println!("Button: X"),
                        Button::North => println!("Button: Y"),
                        Button::LeftTrigger => println!("Bumper: LB"),
                        Button::RightTrigger => println!("Bumper: RB"),
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        // 2. Poll State (Sticks)
        for (_id, gamepad) in gilrs.gamepads() {
            let lx = gamepad.axis_data(Axis::LeftStickX).map(|a| a.value()).unwrap_or(0.0);
            let ly = gamepad.axis_data(Axis::LeftStickY).map(|a| a.value()).unwrap_or(0.0);
            let rx = gamepad.axis_data(Axis::RightStickX).map(|a| a.value()).unwrap_or(0.0);
            let ry = gamepad.axis_data(Axis::RightStickY).map(|a| a.value()).unwrap_or(0.0);



            // Only print if the stick is moved past a small deadzone (0.1)
            if lx.abs() > 0.1 || ly.abs() > 0.1 {
                println!("Left Stick  -> X: {:>5.2}, Y: {:>5.2}", lx, ly);
            }
            if rx.abs() > 0.1 || ry.abs() > 0.1 {
                println!("Right Stick -> X: {:>5.2}, Y: {:>5.2}", rx, ry);
            }
        }

        // Sleep briefly to prevent 100% CPU usage
        thread::sleep(Duration::from_millis(16));
    }
}

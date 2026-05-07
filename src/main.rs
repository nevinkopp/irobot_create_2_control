use gilrs::{Gilrs, Button, Axis, Event, EventType};
use std::thread;
use std::time::Duration;
use serialport::SerialPort;

// Direct drive function takes in serial port and the left and right values from the gamepad. 
fn send_drive_direct(port: &mut dyn SerialPort, left: i16, right: i16) {
    let l_bytes = left.to_be_bytes();
    let r_bytes = right.to_be_bytes();
    let cmd = [145, r_bytes[0], r_bytes[1], l_bytes[0], l_bytes[1]];
    let _ = port.write_all(&cmd);
}

fn main() {

    let port_name = "/dev/ttyUSB0"; // Change this to your serial port
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open serial port");

    // Songs/Beeps 

    let happy = [140, 0, 4, 72, 4, 76, 4, 79, 4, 84, 8];
    port.write_all(&happy).unwrap();

    let sad = [140, 1, 3, 60, 12, 55, 12, 48, 24];
    port.write_all(&sad).unwrap();

    let curious = [140, 2, 4, 75, 4, 0, 2, 75, 4, 82, 12];
    port.write_all(&curious).unwrap();

    let panic = [140, 3, 6, 88, 2, 90, 2, 88, 2, 90, 2, 88, 2, 93, 16];
    port.write_all(&panic).unwrap();

    //let beep = [140, 0, 1, 60, 16]; // simple beep
    //port.write_all(&beep).unwrap();

    // let b_day_0 = [ // happy bday part 1
    //     140, 0, 12, 
    //     67, 24, 67, 8, 69, 32, 67, 32, 72, 32, 71, 64, 
    //     67, 24, 67, 8, 69, 32, 67, 32, 74, 32, 72, 64
    // ];
    //port.write_all(&b_day_0).unwrap();
        
        
    // let b_day_1 = [ // happy bday part 2
    //     140, 1, 13, 
    //     67, 24, 67, 8, 79, 32, 76, 32, 72, 32, 71, 32, 69, 64, 
    //     77, 24, 77, 8, 76, 32, 72, 32, 74, 32, 72, 64
    // ];
    //port.write_all(&b_day_1).unwrap();

    port.write_all(&[128]).unwrap();
    thread::sleep(Duration::from_millis(50));
    port.write_all(&[131]).unwrap(); // safe mode
    println!("Robot Initialized on Serial Port: {}", port_name);

    
    let mut gilrs = Gilrs::new().unwrap();
    println!("Gamepad Active...");

    loop {
        // 1. Button Events
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::South =>  {
                            println!("Button: South");
                            port.write_all(&[141, 0]).unwrap(); // happy beep
                        },
                        Button::East => {
                            println!("Button: East");
                            port.write_all(&[141, 1]).unwrap(); // sad beed
                        },
                        Button::West =>  {
                            println!("Button: West");
                            port.write_all(&[141, 2]).unwrap(); // curious beep
                        },
                        Button::North => {
                            println!("Button: North");
                            port.write_all(&[141, 3]).unwrap(); // panic beep
                        },
                        Button::LeftTrigger => println!("Bumper: LB"),
                        Button::RightTrigger => println!("Bumper: RB"),
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        // 2. Stick Positions
        for (_id, gamepad) in gilrs.gamepads() {
            let lx = gamepad.axis_data(Axis::LeftStickX).map(|a| a.value()).unwrap_or(0.0);
            let ly = gamepad.axis_data(Axis::LeftStickY).map(|a| a.value()).unwrap_or(0.0);
            let rx = gamepad.axis_data(Axis::RightStickX).map(|a| a.value()).unwrap_or(0.0);
            let ry = gamepad.axis_data(Axis::RightStickY).map(|a| a.value()).unwrap_or(0.0);

            let (mut left, mut right) = if ly.abs() > 0.1 || rx.abs() > 0.1 {
                let speed = ly * 500.0;
                let turn = rx * 250.0;
                (speed + turn, speed - turn)
            } else {
                (0.0, 0.0)
            };

            left = left.clamp(-500.0, 500.0);
            right = right.clamp(-500.0, 500.0);

            send_drive_direct(&mut *port, left as i16, right as i16);

            // Only print if the stick is moved past a small deadzone (0.1)
             if lx.abs() > 0.1 || ly.abs() > 0.1 {
                 println!("Left Stick  -> X: {:>5.2}, Y: {:>5.2}", lx, ly);
             }
             if rx.abs() > 0.1 || ry.abs() > 0.1 {
                 println!("Right Stick -> X: {:>5.2}, Y: {:>5.2}", rx, ry);
             }
        }


        
        thread::sleep(Duration::from_millis(20));


    }
}




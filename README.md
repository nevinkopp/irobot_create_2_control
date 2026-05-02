irobot_create_2_control is a rust crate that you can use to control your Create 2 with a gamepad. It uses gilrs and the iRobot Create 2 Open Interface (OI) for Roomba 600 series robots. 

# How to use

## Example hardware
* iRobot Roomba 600 series (required)
* Raspberry Pi (or other SBC)
* USB to 8 pin mini din cable (recommended)
* Battery pack (for the Pi)
* Gamepad (I used an Xbox controller)

## Software setup
* If you are using the gamepad over bluetooth, make sure to add it to the pi through the bluetooth settings.
* SSH into the pi using "ssh username@ip_address".
* If you don't know the pi's ip, run "hostname -I" to get it. 
* Install rust (if not already installed), follow the official rust website for installation.
* Clone this repo to your pi.
* Cd into the cloned repo.
* Currently, the program assumes "/dev/ttyUSB0" for the robot, I will add in CLI arguments later for on the fly adjustments.
* Run "cargo run" to start the program.  
* If successful, you will see a message when the gamepad and robot are both initialized.
* The left stick y axis controls speed, the right stick x axis spins the robot cwise or ccwise. 
* Currently, pressing the south button will make the robot make a beep sound but you can add your own songs and other functions to the other buttons (refer to the OI for full functionality).
* I will probably be updating this here and there or whenever the mood strikes. Simply run "git pull" from within the directory to update. 

use i2cdev::linux::LinuxI2CDevice;
use i2cdev::core::I2CDevice;

type ControlError<E> = Result<(), E>;

const MOTOR_ADDRESS: u16 = 0x2B;

#[repr(u8)]
enum Register {
    MotorControl = 0x01,           
    ServoControl = 0x02,           
    WQ2812All = 0x03,              
    WQ2812Alone = 0x04,            
    IRSwitch = 0x05,               
    BeepSwitch = 0x06,             
    UltrasonicSwitch = 0x07,       
    WQ2812BrightnessAll = 0x08,    
    WQ2812BrightnessAlone = 0x09,  
}

// 0 - 3
#[repr(u8)]
enum Motor {
    ForwardLeft = 0,
    ForwardRight = 1,
    BackwardLeft = 2,
    BackwardRight = 3,
}

#[repr(u8)]
enum MotorDirection {
    Forward = 1,
    Reverse = 0,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaspbotRegister {
        // Ctrl_Car, Ctrl_Muto
        // Ctrl_Servo
         // Ctrl_WQ2812_ALL
         // Ctrl_WQ2812_Alone
          // Ctrl_IR_Switch
          // Ctrl_BEEP_Switch
         // Ctrl_Ulatist_Switch
         // Ctrl_WQ2812_brightness_ALL
         // Ctrl_WQ2812_brightness_Alone
}
fn main() {

    let mut device = match LinuxI2CDevice::new("/dev/i2c-1", MOTOR_ADDRESS) {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to get the i2c device: {}", e);
            return;
        }
    };



}

fn forward(&mut device: LinuxI2CDevice) -> ControlError<> {
    for 
    match device.smbus_write_block_data(
        Register::Motor as u8, &[Motor::ForwardLeft as u8, MotorDirection::Reverse as u8, 255u8],
    ) {
        _ => (),
        Err(e) => {
            println!("{}", e);
        }
    }
}

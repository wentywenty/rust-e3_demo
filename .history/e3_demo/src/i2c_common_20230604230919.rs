use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", 0x2E)?;
    dev.smbus_write_byte_data(0x03, [value])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..2 { // 扫描第0条和第1条I2C总线
        println!("Scanning I2C bus {}", i);
        for addr in 0..128 { // 扫描地址0~127
            let mut dev = match LinuxI2CDevice::new(format!("/dev/i2c-{}", i), addr) {
                Ok(dev) => dev,
                Err(_) => continue, // 如果设备不存在，则继续扫描下一个地址
            };
            let res = dev.smbus_read_byte();
            if res.is_ok() {
                println!("    Address 0x{:02X} - Device found!", addr);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", 0x2E)?;
    let value = dev.smbus_read_byte_data(0x03)?;
    println!("The value of register 0x03 is: {}", value);
    Ok(())
}


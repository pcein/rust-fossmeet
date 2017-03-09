#![feature(lang_items)]
#![no_std]
#![feature(asm)]
#![crate_type="staticlib"]

mod gpio;
mod runtime;

// Prevent the code from getting optimized
// away be the compiler when opt-level=2
#[inline(never)]
fn delay(n: u32) {
   for _ in 0..n {
        unsafe { asm!("nop" :::: "volatile") }
    }
}

#[no_mangle] pub extern "C" fn main()
{
    gpio::port_enable(gpio::SYSCTL_PERIPH_GPIOF);

    gpio::port_output(gpio::GPIO_PORTF_BASE, 
                      gpio::GPIO_PIN_1);

    loop {
        gpio::port_write(gpio::GPIO_PORTF_BASE, gpio::GPIO_PIN_1, 
                         gpio::GPIO_PIN_1);

        delay(500000);
        
        gpio::port_write(gpio::GPIO_PORTF_BASE, gpio::GPIO_PIN_1, 
                         0);
        
        delay(500000);
    }
           
}


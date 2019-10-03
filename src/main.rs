#![no_std]
#![no_main]


//openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg


// pick a panicking behavior
//extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprintln};
//use stm32f3::stm32f303;
use alt_stm32f30x_hal::rcc::RccExt;
use alt_stm32f30x_hal::gpio::*;
//use alt_stm32f30x_hal::device::FLASH;
use embedded_hal::digital::v2::{OutputPin,InputPin};

//use core::mem::MaybeUninit;
//#[link_section = ".program_num"]
//static mut USR_PROG: u16 = MaybeUninit::uninit();

//#[link_section = ".program_start"]
//#[no_mangle]
//extern "C" fn prog() -> ! {
//
//    loop{}
//}


//#[no_mangle]
//static PROGRAM_START: unsafe extern "C" fn() -> ! = prog;


#[entry]
fn main() -> ! {

    hprintln!("Bootloader").unwrap();

    /*let p = alt_stm32f30x_hal::pac::Peripherals::take().unwrap();
 
    let mut rcc = p.RCC.constrain();
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

    //let mut flash = p.FLASH;

    let mut led: PBx<PullNone,Output<PushPull,LowSpeed>> = gpiob
        .pb7
        .output()
        .downgrade()
        .into();
    
    */

    /*unsafe {
        write_flash(&mut flash, &mut USR_PROG, 34);

    }*/
    
    //led.set_high().unwrap();

    unsafe {
        //let address = 0x08010000usize;
        //let r: extern "C" fn() -> ! = address as *const extern "C" fn() -> !;
        //r();
        //PROGRAM_START();
        const POINTER: usize = 0x08010160;
        let function = core::mem::transmute::<usize, extern "C" fn() -> !>(POINTER);
        
        cortex_m::register::msp::write(0x00000000);
        /*core::ptr::write_volatile(
                MSP,0x00000000,
            );*/
        function();

    }

}


/*fn write_flash(mut flash: &mut FLASH, location: *mut u16, data: u16) {

    clear_flash(&mut flash, location as u32);

    while flash.sr.read().bsy().bit_is_set() {}

    unsafe {
        flash.keyr.write(|w| {
            w.bits(0x45670123)
        });
        flash.keyr.write(|w| {
            w.bits(0xCDEF89AB)
        });

        flash.cr.write(|w| {
            w.pg().set_bit()
        });
    }

        // * location
        //let mut x = 0;
        
        //let z = 0;

        unsafe {
            //use core::ptr;
            //let y = &mut USR_PROG as *mut u16;
            //ptr::write_volatile(y, 0xAAAA as u16);
            //Can only set to zero, not to one
            * location = data;
        }
        

        asm::nop();

        while flash.sr.read().bsy().bit_is_set() {}
        
    unsafe {

        flash.sr.write(|w| {
            w.eop().clear_bit()
        });

        flash.cr.write(|w| {
            w.pg().clear_bit();
            w.lock().set_bit()
        });

    }

}

fn clear_flash(flash: &mut FLASH, location: u32) {
    unsafe {

        flash.keyr.write(|w| {
            w.bits(0x45670123)
        });
        flash.keyr.write(|w| {
            w.bits(0xCDEF89AB)
        });

        flash.cr.write(|w| {
            //w.per().set_bit()
            w.bits(0x2)
        });

        flash.ar.write(|w| {
            //w.bits(62)
            w.bits(location)
        });

        flash.cr.write(|w| {
            w.per().set_bit();
            w.strt().set_bit()
            //w.bits(0x42)
        });

        asm::nop();

        while flash.sr.read().bsy().bit_is_set() {}

        if flash.sr.read().eop().bit_is_set() {

            flash.sr.write(|w| {
                w.eop().clear_bit()
            });

            flash.cr.write(|w| {
                w.per().clear_bit()
            });
        }

        flash.cr.write(|w| {
                w.lock().set_bit()
            });

    }

}*/
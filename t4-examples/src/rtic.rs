#![no_std]
#![no_main]

extern crate teensy4_fcb;

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;

use imxrt_hal::gpio::IntoGpio;

#[rtic::app(device = imxrt_hal, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let device: imxrt_hal::Peripherals = cx.device;

        // Prepare the LED
        //
        // The LED on the Teensy 4 is driven by pad B0_03.
        let mut led = device.iomuxc.gpio_b0_03.alt5().into_gpio().output();
        led.set_high().unwrap();
    }
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            core::sync::atomic::spin_loop_hint();
        }
    }
};

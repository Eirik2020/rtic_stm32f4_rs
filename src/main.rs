// ==== SET-UP ====
// --- Compiler directives ---
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// --- Imports ---
// Debugger output for RTT
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

// RTIC
use rtic::app;
use rtic_monotonics::systick::prelude::*;
systick_monotonic!(Mono, 1000); // Set monotonic time to 1000 Hz, 1 ms resolution.

// --- STM32F4 HAL and driver ---
use stm32f4xx_hal::{
    gpio::{
        Output, 
        PushPull, 
        PA5
    },
};
mod stm32f4_driver;

// --- Import tasks ---
mod tasks;
use crate::tasks::blinky;



// ==== APPLICATION SET-UP ====
#[app(
    device = stm32f4xx_hal::pac,  // This device uses the stm32f4xx_hal Peripheral Access Crate (PAC).
    peripherals = true,           // Auto-initializes the Peripherals struct (dp).
    dispatchers = [SPI1],         // Unused interrupts that RTIC can use internally for software tasks. 
)]
mod app {
    use super::*;
    // ==== RESOURCES ====
    #[shared]
    struct Shared {

    }
    #[local]
    struct Local {
        led: PA5<Output<PushPull>>, // LED pin
        state: bool,                // LED state (ON/OFF)
    }


    // ==== INITIALIZATION ====
    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // --- Set up peripherals ---
        // Initialize device peripherals
        let dp = cx.device;
        let dev = stm32f4_driver::init(dp);

        // Define led pin.
        let led = dev.led;

        // Start SysTick with known clock freq
        Mono::start(cx.core.SYST, dev.clocks.sysclk().to_Hz());

        // Initialize RTT
        rtt_init_print!();
        rprintln!("init");


        // --- Schedule initial tasks ---
        blink::spawn().ok();


        // --- Initialize resources ---
        (
            Shared {}, 
            Local { 
                led, 
                state: false 
            })
    }



    // ==== RTIC Task Definitions ====
    #[task(local = [led, state])] 
    // This task blinks the LED periodically. 
    async fn blink(cx: blink::Context) { // Use context cx to access local and shared resources.
        loop {
            blinky::toggle_pin(cx.local.led, cx.local.state); // Toggle LED on or off 
            rprintln!("BLINK!"); // Debug, task succeeded.
            Mono::delay(1000.millis()).await;
        }
    }
    // ...
}
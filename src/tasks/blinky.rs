// Imports
use embedded_hal::digital::OutputPin;


// Toggles LED pin
pub fn toggle_pin<PIN>(led: &mut PIN, state: &mut bool)
where
    PIN: OutputPin,
{
    if *state {
        let _ = led.set_low();   // Turn OFF
        *state = false;
    } else {
        let _ = led.set_high();  // Turn ON
        *state = true;
    }
}

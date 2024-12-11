#![no_std]
#![no_main]

mod avr;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    avr::_delay_ms(1000);
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

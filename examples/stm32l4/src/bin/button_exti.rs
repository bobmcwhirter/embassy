#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[path = "../example_common.rs"]
mod example_common;
use defmt::panic;
use embassy::executor::Spawner;
use embassy_stm32::dbgmcu::Dbgmcu;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::Peripherals;
use embassy_traits::gpio::{WaitForFallingEdge, WaitForRisingEdge};
use example_common::*;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    unsafe {
        Dbgmcu::enable_all();
    }

    let button = Input::new(p.PC13, Pull::Up);
    let mut button = ExtiInput::new(button, p.EXTI13);

    info!("Press the USER button...");

    loop {
        button.wait_for_falling_edge().await;
        info!("Pressed!");
        button.wait_for_rising_edge().await;
        info!("Released!");
    }
}

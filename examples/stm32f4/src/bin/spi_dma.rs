#![no_std]
#![no_main]
#![feature(trait_alias)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[path = "../example_common.rs"]
mod example_common;
use core::fmt::Write;
use core::str::from_utf8;
use defmt::panic;
use embassy::executor::Spawner;
use embassy_stm32::dbgmcu::Dbgmcu;
use embassy_stm32::spi::{Config, Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::Peripherals;
use embassy_traits::spi::FullDuplex;
use example_common::*;
use heapless::String;

extern crate embassy_stm32f4_examples;
use embassy_stm32f4_examples::bsp;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    info!("Hello World!");

    unsafe {
        Dbgmcu::enable_all();
    }

    /*
    let mut spi = Spi::new(
        p.SPI1,
        p.PB3,
        p.PB5,
        p.PB4,
        p.DMA2_CH3,
        p.DMA2_CH2,
        Hertz(1_000_000),
        Config::default(),
    );
     */

    let mut spi = bsp::spi::new_spi(p);

    for n in 0u32.. {
        let mut write: String<128> = String::new();
        let mut read = [0; 128];
        core::write!(&mut write, "Hello DMA World {}!\r\n", n).unwrap();
        spi.read_write(&mut read[0..write.len()], write.as_bytes())
            .await
            .ok();
        info!("read via spi+dma: {}", from_utf8(&read).unwrap());
    }
}

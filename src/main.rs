#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

pub mod buzzer;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{self, Either};
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::gpio::{Input, Level, Output};
use embassy_rp::peripherals::{
    DMA_CH0, PIN_16, PIN_17, PIN_18, PIN_20, PIN_23, PIN_25, PIN_28, PIO0, PWM_CH1,
};
use embassy_rp::pio::Pio;
use embassy_rp::pwm;
use embassy_time::Duration;
use embedded_io::asynch::Write;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        STATIC_CELL.init_with(move || $val)
    }};
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting program");

    let network_id = "internet xD";
    let network_password = "QwerFdsa";

    let p = embassy_rp::init(Default::default());
    unwrap!(spawner.spawn(buzzer::buzzer_task(
        p.PWM_CH1, p.PIN_16, p.PIN_17, p.PIN_18, p.PIN_20, p.PIN_28
    )));


}

use rpi_pac::gicv2::gicd::*;

pub struct Gicd {
    gicd: rpi_pac::GicdShared,
}
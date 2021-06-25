use register::{
    mmio::{ReadOnly, ReadWrite, WriteOnly},
    register_structs,
};

register_structs! {
    pub RegisterBlock {
        (0x00 => pub gpfsel: [ReadWrite<u32>; 6]),
        (0x18 => _reserved0),
        (0x1C => pub gpset: [WriteOnly<u32>; 2]),
        (0x24 => _reserved1),
        (0x28 => pub gpclr: [WriteOnly<u32>; 2]),
        (0x30 => _reserved2),
        (0x34 => pub gplev: [ReadOnly<u32>; 2]),
        (0x3C => _reserved3),
        (0x40 => pub gpeds: [WriteOnly<u32>; 2]),
        (0x48 => _reserved4),
        (0x4C => pub gpren: [ReadWrite<u32>; 2]),
        (0x54 => _reserved5),
        (0x58 => pub gpfen: [ReadWrite<u32>; 2]),
        (0x60 => _reserved6),
        (0x64 => pub gphen: [ReadWrite<u32>; 2]),
        (0x6C => _reserved7),
        (0x70 => pub gplen: [ReadWrite<u32>; 2]),
        (0x78 => _reserved8),
        (0x7C => pub gparen: [ReadWrite<u32>; 2]),
        (0x84 => _reserved9),
        (0x88 => pub gpafen: [ReadWrite<u32>; 2]),
        (0x90 => _reserved10),
        (0xE4 => pub gpio_pup_pdn_cntrl_reg: [ReadWrite<u32>; 4]),
        (0xF4 => @END),
    }
}

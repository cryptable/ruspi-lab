/// Memory mapping of the Raspberry Pi

/// Raspberry Pi 3 base memory address
#[cfg(feature="raspi3")]
pub const MMIO_BASE: u32   = 0x3F000000;

/// Raspberry Pi 4 base memory address
#[cfg(feature="raspi4")]
pub const MMIO_BASE: u32   = 0xFE000000;

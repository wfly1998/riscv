//! hgatp register

use bit_field::BitField;

/// hgatp register
#[derive(Clone, Copy, Debug)]
pub struct Hgatp {
    bits: usize,
}

impl Hgatp {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Current address-translation scheme
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn mode(&self) -> Mode {
        match self.bits.get_bit(31) {
            false => Mode::Bare,
            true => Mode::Sv32x4,
        }
    }

    /// Current address-translation scheme
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn mode(&self) -> Mode {
        match self.bits.get_bits(60..64) {
            0 => Mode::Bare,
            8 => Mode::Sv39x4,
            9 => Mode::Sv48x4,
            10 => Mode::Sv57x4,
            _ => unreachable!(),
        }
    }

    /// Virtual machine identifier
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn vmid(&self) -> usize {
        self.bits.get_bits(22..29)
    }

    /// Virtual machine identifier
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn vmid(&self) -> usize {
        self.bits.get_bits(44..58)
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "32")]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..22)
    }

    /// Physical page number
    #[inline]
    #[cfg(target_pointer_width = "64")]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..44)
    }
}

/// 32-bit hgatp mode
#[cfg(target_pointer_width = "32")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// No translation or protection
    Bare = 0,
    /// Page-based 32-bit addressing
    Sv32x4 = 1,
}

/// 64-bit hgatp mode
#[cfg(target_pointer_width = "64")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// No translation or protection
    Bare = 0,
    /// Page-based 39-bit addressing
    Sv39x4 = 8,
    /// Page-based 48-bit addressing
    Sv48x4 = 9,
    /// Page-based 57-bit addressing
    Sv57x4 = 10,
}

read_csr_as!(Hgatp, 0x680);
write_csr_as_usize!(0x680);

/// Sets the register to corresponding gstage page table mode, physical page number and virtual
/// machine id.
#[inline]
#[cfg(target_pointer_width = "32")]
pub unsafe fn set(mode: Mode, vmid: usize, ppn: usize) {
    let mut bits = 0usize;
    bits.set_bits(31..32, mode as usize);
    bits.set_bits(22..29, vmid);
    bits.set_bits(0..22, ppn);
    _write(bits);
}

/// Sets the register to corresponding gstage page table mode, physical page number and virtual
/// machine id.
#[inline]
#[cfg(target_pointer_width = "64")]
pub unsafe fn set(mode: Mode, vmid: usize, ppn: usize) {
    let mut bits = 0usize;
    bits.set_bits(60..64, mode as usize);
    bits.set_bits(44..58, vmid);
    bits.set_bits(0..44, ppn);
    _write(bits);
}

//! hvip register

use bit_field::BitField;

/// hvip register
#[derive(Clone, Copy, Debug)]
pub struct Hvip {
    bits: usize,
}

impl Hvip {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Virtual Supervisor Software Interrupt Pending
    #[inline]
    pub fn vssoft(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Virtual Supervisor Timer Interrupt Pending
    #[inline]
    pub fn vstimer(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Virtual Supervisor External Interrupt Pending
    #[inline]
    pub fn vsext(&self) -> bool {
        self.bits.get_bit(10)
    }
}

read_csr_as!(Hvip, 0x645);
set!(0x645);
clear!(0x645);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Pending
    , set_vssoft, clear_vssoft, 1 << 2);
set_clear_csr!(
    /// Virtual Supervisor Timer Interrupt Pending
    , set_vstimer, clear_vstimer, 1 << 6);
set_clear_csr!(
    /// Virtual Supervisor External Interrupt Pending
    , set_vsext, clear_vsext, 1 << 10);

/// Clears all bits of the CSR
#[inline]
pub unsafe fn clear() {
    _clear(usize::MAX);
}

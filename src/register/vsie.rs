//! vsie register

use bit_field::BitField;

/// vsie register
#[derive(Clone, Copy, Debug)]
pub struct Vsie {
    bits: usize,
}

impl Vsie {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Virtual Supervisor Software Interrupt Enable
    #[inline]
    pub fn vssoft(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Virtual Supervisor Timer Interrupt Enable
    #[inline]
    pub fn vstimer(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Virtual Supervisor External Interrupt Enable
    #[inline]
    pub fn vsext(&self) -> bool {
        self.bits.get_bit(10)
    }
}

read_csr_as!(Vsie, 0x204);
set!(0x204);
clear!(0x204);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Enable
    , set_vssoft, clear_vssoft, 1 << 2);
set_clear_csr!(
    /// Virtual Supervisor Timer Interrupt Enable
    , set_vstimer, clear_vstimer, 1 << 6);
set_clear_csr!(
    /// Virtual Supervisor External Interrupt Enable
    , set_vsext, clear_vsext, 1 << 10);

/// Clears all bits of the CSR
#[inline]
pub unsafe fn clear() {
    _clear(usize::MAX);
}

//! hideleg register

use bit_field::BitField;

/// hideleg register
#[derive(Clone, Copy, Debug)]
pub struct Hideleg {
    bits: usize,
}

impl Hideleg {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Virtual Supervisor Software Interrupt Delegate
    #[inline]
    pub fn vssoft(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Virtual Supervisor Timer Interrupt Delegate
    #[inline]
    pub fn vstimer(&self) -> bool {
        self.bits.get_bit(6)
    }

    /// Virtual Supervisor External Interrupt Delegate
    #[inline]
    pub fn vsext(&self) -> bool {
        self.bits.get_bit(10)
    }
}

read_csr_as!(Hideleg, 0x603);
set!(0x603);
clear!(0x603);

set_clear_csr!(
    /// Virtual Supervisor Software Interrupt Delegate
    , set_vsoft, clear_vsoft, 1 << 2);
set_clear_csr!(
    /// Virtual Supervisor Timer Interrupt Delegate
    , set_vtimer, clear_vtimer, 1 << 6);
set_clear_csr!(
    /// Virtual Supervisor External Interrupt Delegate
    , set_vext, clear_vext, 1 << 10);

/// Clears all bits of the CSR
#[inline]
pub unsafe fn clear() {
    _clear(usize::MAX);
}

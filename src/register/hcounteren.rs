//! hcounteren register

use bit_field::BitField;

/// hcounteren register
#[derive(Clone, Copy, Debug)]
pub struct Hcounteren {
    bits: usize,
}

impl Hcounteren {
    /// Supervisor "cycle\[h\]" Enable
    #[inline]
    pub fn cy(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor "time\[h\]" Enable
    #[inline]
    pub fn tm(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Supervisor "instret\[h\]" Enable
    #[inline]
    pub fn ir(&self) -> bool {
        self.bits.get_bit(2)
    }

    /// Supervisor "hpm\[x\]" Enable (bits 3-31)
    #[inline]
    pub fn hpm(&self, index: usize) -> bool {
        assert!((3..32).contains(&index));
        self.bits.get_bit(index)
    }
}

read_csr_as!(Hcounteren, 0x606);
write_csr!(0x606);
set!(0x606);
clear!(0x606);

set_clear_csr!(
/// Guest cycle Enable
    , set_cy, clear_cy, 1 << 0);

set_clear_csr!(
/// Guest time Enable
    , set_tm, clear_tm, 1 << 1);

set_clear_csr!(
/// Guest instret Enable
    , set_ir, clear_ir, 1 << 2);

#[inline]
pub unsafe fn set_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _set(1 << index);
}

#[inline]
pub unsafe fn clear_hpm(index: usize) {
    assert!((3..32).contains(&index));
    _clear(1 << index);
}

/// Clears all bits of the CSR
#[inline]
pub unsafe fn clear() {
    _clear(usize::MAX);
}

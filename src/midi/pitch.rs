pub const C:       u8 = 12;
pub const D:       u8 = C + 2;
pub const E:       u8 = D + 2;
pub const F:       u8 = E + 1;
pub const G:       u8 = F + 2;
pub const A:       u8 = G + 2;
pub const B:       u8 = A + 2;

pub const C_SHARP: u8 = C + 1;
pub const D_FLAT:  u8 = D - 1;
pub const D_SHARP: u8 = D + 1;
pub const E_FLAT:  u8 = E - 1;
pub const F_SHARP: u8 = F + 1;
pub const G_FLAT:  u8 = G - 1;
pub const G_SHARP: u8 = G + 1;
pub const A_FLAT:  u8 = A - 1;
pub const A_SHARP: u8 = A + 1;
pub const B_FLAT:  u8 = B - 1;

pub fn octave(note: u8, oct: u8) -> u8{
    return note + oct*C;
}

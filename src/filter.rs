use cantypes::filter::{CanIdFilter, MaskType};
use cantypes::constants::{STANDARD_FRAME_ID_LENGTH, EXTENDED_FRAME_ID_LENGTH};


#[derive(Debug, PartialEq, Clone)]
pub struct AcceptanceCodeRegister {
    acr0: u8,
    acr1: u8,
    acr2: u8,
    acr3: u8
}

impl AcceptanceCodeRegister {
    pub fn new(acr0: u8, acr1: u8, acr2: u8, acr3: u8) -> Self {
        AcceptanceCodeRegister { acr0, acr1, acr2, acr3 }
    }

    pub fn from_single_filter<T: CanIdFilter>(value: &T) -> Self {
        match value.mask_type() {
            MaskType::Standard => {
                AcceptanceCodeRegister::new(
                    ((value.can_id() >> (STANDARD_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    (((value.can_id() & ((1 << 3) -1 )) << 5) | ((1 << 5) -1)) as u8,
                    ((value.can_id() >> (STANDARD_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    (((value.can_id() & ((1 << 3) -1 )) << 5) | ((1 << 5) -1)) as u8,
                )
            },
            MaskType::Extended => {
                AcceptanceCodeRegister::new(
                    ((value.can_id() >> (EXTENDED_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((value.can_id() >> (EXTENDED_FRAME_ID_LENGTH - 8 - 5)) & 0xFF) << 3) | ((1 << 3) -1)) as u8,
                    ((value.can_id() >> (EXTENDED_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((value.can_id() >> (EXTENDED_FRAME_ID_LENGTH - 8 - 5)) & 0xFF) << 3) | ((1 << 3) -1)) as u8,
                )
            },
        }
    }

    pub fn acr0(&self) -> u8 {
        self.acr0
    }

    pub fn acr1(&self) -> u8 {
        self.acr1
    }

    pub fn acr2(&self) -> u8 {
        self.acr2
    }

    pub fn acr3(&self) -> u8 {
        self.acr3
    }

    pub fn register(&self) -> u32 {
        u32::from(self)
    }
}

impl From<u32> for AcceptanceCodeRegister {
    fn from(value: u32) -> Self {
        AcceptanceCodeRegister {
            acr0: ((value >> 24) & 0xFF) as u8,
            acr1: ((value >> 16) & 0xFF) as u8,
            acr2: ((value >> 8) & 0xFF) as u8,
            acr3: ((value >> 0) & 0xFF) as u8,
        }
    }
}

impl From<&u32> for AcceptanceCodeRegister {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}

impl From<AcceptanceCodeRegister> for u32 {
    fn from(value: AcceptanceCodeRegister) -> Self {
        ((value.acr0 as u32) << 24) | ((value.acr1 as u32) << 16) | ((value.acr2 as u32) << 8) | ((value.acr3 as u32) << 0)
    }
}

impl From<&AcceptanceCodeRegister> for u32 {
    fn from(value: &AcceptanceCodeRegister) -> Self {
        ((value.acr0() as u32) << 24) | ((value.acr1() as u32) << 16) | ((value.acr2() as u32) << 8) | ((value.acr3() as u32) << 0)
    }
}

#[derive(Debug, PartialEq)]
pub struct AcceptanceMaskRegister {
    amr0: u8,
    amr1: u8,
    amr2: u8,
    amr3: u8
}

impl AcceptanceMaskRegister {
    pub fn new(amr0: u8, amr1: u8, amr2: u8, amr3: u8) -> Self {
        AcceptanceMaskRegister { amr0, amr1, amr2, amr3 }
    }

    pub fn from_single_filter<T: CanIdFilter>(value: &T) -> Self {
        match value.mask_type() {
            MaskType::Standard => {
                AcceptanceMaskRegister::new(
                    ((!value.mask() >> (STANDARD_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((!value.mask() & ((1 << 3) - 1))) << 5) | ((1 << 5) -1)) as u8,
                    ((!value.mask() >> (STANDARD_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((!value.mask() & ((1 << 3) - 1))) << 5) | ((1 << 5) -1)) as u8,
                )
            },
            MaskType::Extended => {
                AcceptanceMaskRegister::new(
                    ((!value.mask() >> (EXTENDED_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((!value.mask() >> (EXTENDED_FRAME_ID_LENGTH - 8 - 5)) & 0xFF) << 3) | ((1 << 3) -1)) as u8,
                    ((!value.mask() >> (EXTENDED_FRAME_ID_LENGTH - 8)) & 0xFF) as u8,
                    ((((!value.mask() >> (EXTENDED_FRAME_ID_LENGTH - 8 - 5)) & 0xFF) << 3) | ((1 << 3) -1)) as u8,
                )
            },
        }
    }

    pub fn amr0(&self) -> u8 {
        self.amr0
    }

    pub fn amr1(&self) -> u8 {
        self.amr1
    }

    pub fn amr2(&self) -> u8 {
        self.amr2
    }

    pub fn amr3(&self) -> u8 {
        self.amr3
    }

    pub fn register(&self) -> u32 {
        u32::from(self)
    }
}

impl From<u32> for AcceptanceMaskRegister {
    fn from(value: u32) -> Self {
        AcceptanceMaskRegister {
            amr0: ((value >> 24) & 0xFF) as u8,
            amr1: ((value >> 16) & 0xFF) as u8,
            amr2: ((value >> 8) & 0xFF) as u8,
            amr3: ((value >> 0) & 0xFF) as u8,
        }
    }
}

impl From<&u32> for AcceptanceMaskRegister {
    fn from(value: &u32) -> Self {
        Self::from(*value)
    }
}

impl From<AcceptanceMaskRegister> for u32 {
    fn from(value: AcceptanceMaskRegister) -> Self {
        ((value.amr0 as u32) << 24) | ((value.amr1 as u32) << 16) | ((value.amr2 as u32) << 8) | ((value.amr3 as u32) << 0)
    }
}

impl From<&AcceptanceMaskRegister> for u32 {
    fn from(value: &AcceptanceMaskRegister) -> Self {
        ((value.amr0() as u32) << 24) | ((value.amr1() as u32) << 16) | ((value.amr2() as u32) << 8) | ((value.amr3() as u32) << 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cantypes::filter::{StandardCanIdFilter, ExtendedCanIdFilter, CanIdFilter};

    #[test]
    fn transform_acceptance_code_register_001() {
        let value = 0xAABBCCDDu32;
        let register = AcceptanceCodeRegister::from(value);
        let from_value = u32::from(&register);
        assert!(value == from_value);
        assert!(value == register.register());
        assert!(from_value == register.register());
    }

    #[test]
    fn transform_acceptance_mask_register_001() {
        let value = 0x12345678u32;
        let register = AcceptanceMaskRegister::from(value);
        let from_value = u32::from(&register);
        assert!(value == from_value);
        assert!(value == register.register());
        assert!(from_value == register.register());
    }

    #[test]
    fn acceptance_code_register_from_std_filter() {
        let filter = StandardCanIdFilter::from_can_id(0x601);
        let code_register = AcceptanceCodeRegister::from_single_filter(&filter);
        println!("{:08X}", u32::from(&code_register));
        assert_eq!(u32::from(code_register), 0xC0_3F_C0_3Fu32);
    }

    #[test]
    fn acceptance_mask_register_from_std_filter() {
        let filter = StandardCanIdFilter::from_can_id(0x601);
        let code_register = AcceptanceMaskRegister::from_single_filter(&filter);
        assert_eq!(u32::from(code_register), 0x00_1F_00_1Fu32);
    }

    #[test]
    fn acceptance_code_register_from_ext_filter() {
        let filter = ExtendedCanIdFilter::from_can_id(0x18_DA_00_00);
        let code_register = AcceptanceCodeRegister::from_single_filter(&filter);
        assert_eq!(u32::from(code_register), 0xC6_D7_C6_D7u32);
    }

    #[test]
    fn acceptance_mask_register_from_ext_filter() {
        let filter = ExtendedCanIdFilter::from_can_id(0x18_DA_00_00);
        let code_register = AcceptanceMaskRegister::from_single_filter(&filter);
        assert_eq!(u32::from(code_register), 0x00_07_00_07u32);
    }
}
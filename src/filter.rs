
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

struct Filter<'a> {
    can_ids: &'a [u32],
    acceptance_code_register: AcceptanceCodeRegister,
    acceptance_mask_register: AcceptanceMaskRegister
}

impl Filter<'_> {
    pub fn new(can_ids: &'static [u32]) -> Self {
        if can_ids.len() == 0 {
            return Self {
                can_ids: can_ids,
                acceptance_code_register: 0x00_00_00_00.into(),
                acceptance_mask_register: 0xFF_FF_FF_FF.into()
            };
        }



        Self {
            can_ids: can_ids,
            acceptance_code_register: 0.into(),
            acceptance_mask_register: 0.into()
        }
    }

    pub fn acceptance_code_register(&self) -> u32 {
        u32::from(&self.acceptance_code_register)
    }

    pub fn acceptance_mask_register(&self) -> u32 {
        u32::from(&self.acceptance_mask_register)
    }

    pub fn check(&self, can_id: u32) -> bool {
        match self.can_ids.len() {
            0 => true,
            1 => self.can_ids[0] == can_id,
            n => {
                false
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

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

    // #[test]
    // fn transform_acceptance_mask_register_001() {
    //     let array = [0x6]
    //     let filter = Filter::new()


    //     let value = 0x12345678u32;
    //     let register = AcceptanceMaskRegister::from(value);
    //     let from_value = u32::from(&register);
    //     assert!(value == from_value);
    //     assert!(value == register.register());
    //     assert!(from_value == register.register());
    // }
}
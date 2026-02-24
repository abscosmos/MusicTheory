use crate::set::forte::{NewSetClassError, SetClass};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InversionForm {
    A,
    B
}

pub struct SetClassForm {
    set_class: SetClass,
    inversion: Option<InversionForm>,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NewSetClassFormError {
    #[error(transparent)]
    InvalidSetClass(#[from] NewSetClassError),
    #[error("Inversion (or lack thereof) invalid for set class")]
    InvalidInversion,
}

impl SetClassForm {
    pub const fn new(cardinality: u8, index: u8, inversion: Option<InversionForm>) -> Result<Self, NewSetClassFormError> {
        // FIXME(const)
        let set_class = match SetClass::new(cardinality, index) {
            Ok(set_class) => set_class,
            Err(err) => return Err(NewSetClassFormError::InvalidSetClass(err))
        };

        // TODO: this is a second lookup
        let prime_form = set_class.prime_form();

        if inversion.is_none() != prime_form.inversionally_symmetric() {
            return Err(NewSetClassFormError::InvalidInversion);
        }

        Ok(Self { set_class, inversion })
    }
    
    pub fn inversionally_symmetric(self) -> bool {
        self.inversion.is_none()
    }

    #[inline]
    pub const fn set_class(self) -> SetClass {
        self.set_class
    }
}
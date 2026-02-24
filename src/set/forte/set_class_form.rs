use crate::set::forte::{NewSetClassError, SetClass};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum InversionForm {
    A,
    B
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

        if inversion.is_none() != set_class.is_inversionally_symmetric() {
            return Err(NewSetClassFormError::InvalidInversion);
        }

        Ok(Self { set_class, inversion })
    }

    pub fn is_inversionally_symmetric(self) -> bool {
        self.inversion.is_none()
    }

    #[inline]
    pub const fn set_class(self) -> SetClass {
        self.set_class
    }
}
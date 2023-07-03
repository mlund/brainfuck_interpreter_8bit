use ufmt_stdio::*;

#[derive(Debug)]
pub enum BrainFuckError {
    InputNotProvidedError,
    InputNotEnoughError,
    LoopNotClosedError(usize),
}

impl ufmt_stdio::uDisplay for BrainFuckError {
    fn fmt<W>(&self, f: &mut ufmt_stdio::ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt_stdio::uWrite + ?Sized,
    {
        match self {
            Self::InputNotProvidedError => {
                uwrite!(f, "Input required but not provided")
            }
            Self::InputNotEnoughError => {
                uwrite!(f, "Input is given but not enough")
            }
            Self::LoopNotClosedError(index) => {
                uwrite!(f, "Can't close loop, index: {}", index)
            }
        }
    }
}

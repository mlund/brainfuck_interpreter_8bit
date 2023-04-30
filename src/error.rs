#[derive(Debug)]
pub enum BrainFuckError {
    InputNotProvidedError,
    InputNotEnoughError,
    LoopNotClosedError(usize),
}

impl std::fmt::Display for BrainFuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputNotProvidedError => {
                write!(f, "Input required but not provided")
            }
            Self::InputNotEnoughError => {
                write!(f, "Input is given but not enough")
            }
            Self::LoopNotClosedError(index) => {
                write!(f, "Can't close loop, index: {}", index)
            }
        }
    }
}

impl std::error::Error for BrainFuckError {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_error_print() {
        let e = BrainFuckError::InputNotProvidedError;
        assert_eq!(e.to_string(), "Input required but not provided");

        let e = BrainFuckError::InputNotEnoughError;
        assert_eq!(e.to_string(), "Input is given but not enough");

        let e = BrainFuckError::LoopNotClosedError(3);
        assert_eq!(e.to_string(), "Can't close loop, index: 3");
    }
}

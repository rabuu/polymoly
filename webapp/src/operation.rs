use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Gcd,
}

impl Operation {
    pub fn operand_ring_type(&self) -> OperandRingType {
        match self {
            Operation::Add => OperandRingType::Normal,
            Operation::Sub => OperandRingType::Normal,
            Operation::Mul => OperandRingType::Normal,
            Operation::Div => OperandRingType::Field,
            Operation::Gcd => OperandRingType::Euclidean,
        }
    }
}

impl From<String> for Operation {
    fn from(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "gcd" => Self::Gcd,
            _ => unreachable!(),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Sub => write!(f, "sub"),
            Operation::Mul => write!(f, "mul"),
            Operation::Div => write!(f, "div"),
            Operation::Gcd => write!(f, "gcd"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandRingType {
    Normal,
    Field,
    Euclidean,
}

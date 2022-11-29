use crate::filled_string::FilledString;

pub enum ProofValue {
    Unproven,
    Proven,
    OnlyEvidence,
}

enum Text {
    Plain(FilledString),
}

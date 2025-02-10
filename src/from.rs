use crate::{AddressRecord, InputRecord, Label, OutputRecord, TransactionRecord};

impl From<TransactionRecord> for Label {
    fn from(value: TransactionRecord) -> Self {
        Self::Transaction(value)
    }
}

impl From<AddressRecord> for Label {
    fn from(value: AddressRecord) -> Self {
        Self::Address(value)
    }
}

impl From<InputRecord> for Label {
    fn from(value: InputRecord) -> Self {
        Self::Input(value)
    }
}

impl From<OutputRecord> for Label {
    fn from(value: OutputRecord) -> Self {
        Self::Output(value)
    }
}

use std::cmp::Reverse;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Order {
    pub price: u64,
    pub amount: u64,
    pub side: Side,
    pub signer: String,
}

impl Order {
    pub fn into_partial_order(self, ordinal: u64, remaining: u64) -> PartialOrder {
        let Order {
            price,
            amount,
            side,
            signer,
        } = self;
        PartialOrder {
            price,
            amount,
            remaining,
            side,
            signer,
            ordinal,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Ord)]
pub struct PartialOrder {
    pub price: u64,
    pub amount: u64,
    pub remaining: u64,
    pub side: Side,
    pub signer: String,
    pub ordinal: u64,
}

impl PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.ordinal).partial_cmp(&Reverse(other.ordinal))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Debug, Ord)]
pub struct Receipt {
    pub ordinal: u64,
    pub matches: Vec<PartialOrder>,
}

impl PartialOrder {
    pub fn take_from(pos: &mut PartialOrder, take: u64, price: u64) -> PartialOrder {
        pos.remaining -= take;
        let mut new = pos.clone();
        new.amount = take;
        new.price = price;
        new
    }
}

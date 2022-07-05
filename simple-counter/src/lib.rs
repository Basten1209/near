use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct State {
    count: u64,
}

#[near_bindgen]
impl State {
    #[init]
    pub fn new(count: u64) -> Self {}

    pub fn get_num(&self) -> u64 {
        return self.count;
    }

    pub fn increment(&mut self) {
        self.count += 1;
        let log_message = format!("Increased number to {}", self.count);
        env::log(log_message.as_bytes());
    }
    
    pub fn decrement(&mut self) {
        self.count -= 1;
        let log_message = format!("Decreased number to {}", self.count);
        env::log(log_message.as_bytes());
    }

    pub fn reset(&mut self) {
        self.count = 0;
        env::log(b"Reset counter to zero");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn increment() {
        let mut contract = State { count: 0 };
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = State { count: 1 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset() {
        let mut contract = State { count: 100 };
        contract.reset();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = State {count:u64::max_value()};
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panic_on_underflow() {
        let mut contract = State {count:u64::min_value()};
        contract.decrement();
    }
}

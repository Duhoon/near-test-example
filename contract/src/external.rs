use crate::*;

#[ext_contract(promise_contract)]
trait ExtContract{
    fn extern_set_balance(&mut self, _balance: U128);
}
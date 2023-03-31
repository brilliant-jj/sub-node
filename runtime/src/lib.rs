use pallet_template as claims;
...
parameter_types! {
    pub const ClaimMaxCount: u32 = 10;
}
impl claims::Trait for Runtime {
    type Event = Event;
    type ClaimMaxCount = ClaimMaxCount; // 增加新的 Trait 参数
}
...
impl<T: Trait> system::offchain::CreateSignedTransaction<T> for Module<T> {
    ...
    let claim_hash = <Module<T>>::random_hash();
    let call = Call::ClaimsModule(claims::Call::create_claim(claim_hash.clone().as_ref().to_vec()));
    ...
    let transfer_call =
        Call::ClaimsModule(claims::Call::transfer_claim(to.clone(), claim_hash.clone()));
    dispatch_signed(custom_signed_payload(Some(to.clone()), to.key.clone()), transfer_call)?;
    ...
}

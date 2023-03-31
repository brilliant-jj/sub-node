use support::{ensure};
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct Transfer<AccountId, Hash> {
    pub from: AccountId,
    pub to: AccountId,
    pub claim_hash: Hash,
}

impl<T: Trait> Module<T> {
    pub fn transfer_claim(
        origin: T::Origin,
        to: T::AccountId,
        claim_hash: T::Hash,
    ) -> Result {
        let sender = ensure_signed(origin)?;
        // 判断存证是否存在
        let claim = Self::get_claim(&claim_hash);
        ensure!(claim.is_some(), "该存证不存在");
        
        let (owner, _, _) = Proofs::<T>::get(&claim_hash);
        ensure!(sender == owner, "只有存证的所有者才能转移存证.");
        // 执行存证转移操作，并更新存证信息
        Proofs::<T>::insert(&claim_hash, (to.clone(), claim.unwrap().claim, <system::Module<T>>::block_number()));
        Self::deposit_event(RawEvent::ClaimTransfered(sender, to.clone(), claim_hash.clone()));
        Ok(())
    }
}

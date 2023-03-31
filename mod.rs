pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type ClaimMaxCount: Get<u32>;
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ClaimCreated(AccountId, T::Hash),
        ClaimRevoked(AccountId, T::Hash),
        ClaimTransfered(AccountId, AccountId, T::Hash),
    }
);

impl<T: Trait> Module<T> {
    ...
    // 新增转移存证的函数调用
    pub fn transfer_claim(
        origin: T::Origin,
        to: T::AccountId,
        claim_hash: T::Hash,
    ) -> Result {
      ...
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        #[weight = 0]
        pub fn create_claim(origin, claim: Vec<u8>) -> Result {
            ...
        }
        #[weight = 0]
        pub fn revoke_claim(origin, claim: T::Hash) -> Result {
            ...
        }
        #[weight = 0]
        // 新增转移存证的函数调用
        pub fn transfer_claim(origin, to: T::AccountId, claim_hash: T::Hash) -> Result {
            Module::<T>::transfer_claim(origin, to, claim_hash)
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Claims {
        // 修改 Proofs 的类型，增加了持有人转移到其他人的操作
        pub Proofs get(proof): map T::Hash => (T::AccountId, Vec<u8>, T::BlockNumber);
    }
}

use frame_support::{decl_module, decl_storage, decl_event, dispatch:: DispatchResult, StorageMap, ensure};
use system::ensure_signed;
use sp_std::vec::Vec;

pub trait Trait{

    type Event: From<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        Proofs: map Vec<u8> => (T::AccountId, T::BlockNumber);
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin T::origin{
     
        fn deposit_event() = default;
        
        fn claim(origin, proof: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::exists(&proof), "This proof has already been claimed");

            let current_block = <system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (sender.clone(), current_block));

            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
        ClaimCreated(AccountId, Vec<u8>),
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    
    use sp_core::H256;
    use frame_support::{impl_outer_origin, assert_ok, parameter_types, weights::Weight};
    use sp_runtime::{
        traits::{BlakTwo256, IdentityLookup}, testing::Header, Perbill,
    }
};

impl_outer_origin! {
    pub enum Origin for Test {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
 }

impl Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlackTwo256;
    type AccountId = u64
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;

}

impl Trait for Test {
    type Event = ();
}

type TemplateModule = Module<Test>;

fn new_test() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

#[test]
fn it_tests_users_make_claims() {
    new_test_ext().execute_with(|| {
        let hash: Vec<u8> = [0,1,2].to_vec();
        assert_ok!(TemplateModule::it_tests_users_make_claims(Origin::signed(1), hash.clone()));
        assert_eq!(<Proofs<Test>>::get(hash), (1,1));
    });
}

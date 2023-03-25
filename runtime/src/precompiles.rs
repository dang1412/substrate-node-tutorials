use pallet_evm::{PrecompileResult, PrecompileSet, PrecompileHandle};
use sp_core::H160;
use sp_std::marker::PhantomData;

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> sp_std::vec::Vec<H160> {
		sp_std::vec![1, 2, 3, 4, 5, 1024, 1025]
			.into_iter()
			.map(|x| hash(x))
			.collect()
	}
}

impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn execute(
		&self,
		_handle: &mut impl PrecompileHandle
	) -> Option<PrecompileResult> {
		None
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().contains(&address)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

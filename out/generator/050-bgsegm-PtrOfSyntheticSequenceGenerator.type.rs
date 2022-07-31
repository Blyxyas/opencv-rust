pub type PtrOfSyntheticSequenceGenerator = core::Ptr<crate::bgsegm::SyntheticSequenceGenerator>;

ptr_extern! { crate::bgsegm::SyntheticSequenceGenerator,
	cv_PtrOfSyntheticSequenceGenerator_delete, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr, cv_PtrOfSyntheticSequenceGenerator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::bgsegm::SyntheticSequenceGenerator, cv_PtrOfSyntheticSequenceGenerator_new }

impl PtrOfSyntheticSequenceGenerator {
	#[inline] pub fn as_raw_PtrOfSyntheticSequenceGenerator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSyntheticSequenceGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bgsegm::SyntheticSequenceGeneratorTraitConst for PtrOfSyntheticSequenceGenerator {
	#[inline] fn as_raw_SyntheticSequenceGenerator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bgsegm::SyntheticSequenceGeneratorTrait for PtrOfSyntheticSequenceGenerator {
	#[inline] fn as_raw_mut_SyntheticSequenceGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSyntheticSequenceGenerator {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSyntheticSequenceGenerator {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


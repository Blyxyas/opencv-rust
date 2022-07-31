pub type PtrOfGPCTrainingSamples = core::Ptr<crate::optflow::GPCTrainingSamples>;

ptr_extern! { crate::optflow::GPCTrainingSamples,
	cv_PtrOfGPCTrainingSamples_delete, cv_PtrOfGPCTrainingSamples_get_inner_ptr, cv_PtrOfGPCTrainingSamples_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::optflow::GPCTrainingSamples, cv_PtrOfGPCTrainingSamples_new }

impl PtrOfGPCTrainingSamples {
	#[inline] pub fn as_raw_PtrOfGPCTrainingSamples(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGPCTrainingSamples(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::optflow::GPCTrainingSamplesTraitConst for PtrOfGPCTrainingSamples {
	#[inline] fn as_raw_GPCTrainingSamples(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::optflow::GPCTrainingSamplesTrait for PtrOfGPCTrainingSamples {
	#[inline] fn as_raw_mut_GPCTrainingSamples(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


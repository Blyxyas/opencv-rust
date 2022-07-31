pub type PtrOfANN_MLP = core::Ptr<dyn crate::ml::ANN_MLP>;

ptr_extern! { dyn crate::ml::ANN_MLP,
	cv_PtrOfANN_MLP_delete, cv_PtrOfANN_MLP_get_inner_ptr, cv_PtrOfANN_MLP_get_inner_ptr_mut
}

impl PtrOfANN_MLP {
	#[inline] pub fn as_raw_PtrOfANN_MLP(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfANN_MLP(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::ANN_MLPConst for PtrOfANN_MLP {
	#[inline] fn as_raw_ANN_MLP(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::ANN_MLP for PtrOfANN_MLP {
	#[inline] fn as_raw_mut_ANN_MLP(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfANN_MLP {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfANN_MLP {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfANN_MLP {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfANN_MLP {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


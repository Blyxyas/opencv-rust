pub type PtrOfMergeMertens = core::Ptr<dyn crate::photo::MergeMertens>;

ptr_extern! { dyn crate::photo::MergeMertens,
	cv_PtrOfMergeMertens_delete, cv_PtrOfMergeMertens_get_inner_ptr, cv_PtrOfMergeMertens_get_inner_ptr_mut
}

impl PtrOfMergeMertens {
	#[inline] pub fn as_raw_PtrOfMergeMertens(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeMertens(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::MergeMertensConst for PtrOfMergeMertens {
	#[inline] fn as_raw_MergeMertens(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeMertens for PtrOfMergeMertens {
	#[inline] fn as_raw_mut_MergeMertens(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMergeMertens {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMergeMertens {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::MergeExposuresConst for PtrOfMergeMertens {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposures for PtrOfMergeMertens {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


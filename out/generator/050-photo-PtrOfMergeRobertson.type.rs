pub type PtrOfMergeRobertson = core::Ptr<dyn crate::photo::MergeRobertson>;

ptr_extern! { dyn crate::photo::MergeRobertson,
	cv_PtrOfMergeRobertson_delete, cv_PtrOfMergeRobertson_get_inner_ptr, cv_PtrOfMergeRobertson_get_inner_ptr_mut
}

impl PtrOfMergeRobertson {
	#[inline] pub fn as_raw_PtrOfMergeRobertson(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeRobertson(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::MergeRobertsonConst for PtrOfMergeRobertson {
	#[inline] fn as_raw_MergeRobertson(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
	#[inline] fn as_raw_mut_MergeRobertson(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMergeRobertson {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMergeRobertson {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::MergeExposuresConst for PtrOfMergeRobertson {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposures for PtrOfMergeRobertson {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


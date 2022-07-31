pub type PtrOfMergeDebevec = core::Ptr<dyn crate::photo::MergeDebevec>;

ptr_extern! { dyn crate::photo::MergeDebevec,
	cv_PtrOfMergeDebevec_delete, cv_PtrOfMergeDebevec_get_inner_ptr, cv_PtrOfMergeDebevec_get_inner_ptr_mut
}

impl PtrOfMergeDebevec {
	#[inline] pub fn as_raw_PtrOfMergeDebevec(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMergeDebevec(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::MergeDebevecConst for PtrOfMergeDebevec {
	#[inline] fn as_raw_MergeDebevec(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
	#[inline] fn as_raw_mut_MergeDebevec(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMergeDebevec {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMergeDebevec {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::photo::MergeExposuresConst for PtrOfMergeDebevec {
	#[inline] fn as_raw_MergeExposures(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::photo::MergeExposures for PtrOfMergeDebevec {
	#[inline] fn as_raw_mut_MergeExposures(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


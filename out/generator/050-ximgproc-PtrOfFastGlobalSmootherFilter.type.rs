pub type PtrOfFastGlobalSmootherFilter = core::Ptr<dyn crate::ximgproc::FastGlobalSmootherFilter>;

ptr_extern! { dyn crate::ximgproc::FastGlobalSmootherFilter,
	cv_PtrOfFastGlobalSmootherFilter_delete, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr, cv_PtrOfFastGlobalSmootherFilter_get_inner_ptr_mut
}

impl PtrOfFastGlobalSmootherFilter {
	#[inline] pub fn as_raw_PtrOfFastGlobalSmootherFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastGlobalSmootherFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::FastGlobalSmootherFilterConst for PtrOfFastGlobalSmootherFilter {
	#[inline] fn as_raw_FastGlobalSmootherFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::FastGlobalSmootherFilter for PtrOfFastGlobalSmootherFilter {
	#[inline] fn as_raw_mut_FastGlobalSmootherFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFastGlobalSmootherFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFastGlobalSmootherFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


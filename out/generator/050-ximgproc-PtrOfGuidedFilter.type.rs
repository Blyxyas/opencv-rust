pub type PtrOfGuidedFilter = core::Ptr<dyn crate::ximgproc::GuidedFilter>;

ptr_extern! { dyn crate::ximgproc::GuidedFilter,
	cv_PtrOfGuidedFilter_delete, cv_PtrOfGuidedFilter_get_inner_ptr, cv_PtrOfGuidedFilter_get_inner_ptr_mut
}

impl PtrOfGuidedFilter {
	#[inline] pub fn as_raw_PtrOfGuidedFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGuidedFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::GuidedFilterConst for PtrOfGuidedFilter {
	#[inline] fn as_raw_GuidedFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::GuidedFilter for PtrOfGuidedFilter {
	#[inline] fn as_raw_mut_GuidedFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGuidedFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGuidedFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


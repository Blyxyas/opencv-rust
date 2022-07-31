pub type PtrOfContourFitting = core::Ptr<crate::ximgproc::ContourFitting>;

ptr_extern! { crate::ximgproc::ContourFitting,
	cv_PtrOfContourFitting_delete, cv_PtrOfContourFitting_get_inner_ptr, cv_PtrOfContourFitting_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::ximgproc::ContourFitting, cv_PtrOfContourFitting_new }

impl PtrOfContourFitting {
	#[inline] pub fn as_raw_PtrOfContourFitting(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfContourFitting(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::ContourFittingTraitConst for PtrOfContourFitting {
	#[inline] fn as_raw_ContourFitting(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::ContourFittingTrait for PtrOfContourFitting {
	#[inline] fn as_raw_mut_ContourFitting(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfContourFitting {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfContourFitting {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


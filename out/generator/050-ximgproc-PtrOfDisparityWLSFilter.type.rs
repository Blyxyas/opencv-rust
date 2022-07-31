pub type PtrOfDisparityWLSFilter = core::Ptr<dyn crate::ximgproc::DisparityWLSFilter>;

ptr_extern! { dyn crate::ximgproc::DisparityWLSFilter,
	cv_PtrOfDisparityWLSFilter_delete, cv_PtrOfDisparityWLSFilter_get_inner_ptr, cv_PtrOfDisparityWLSFilter_get_inner_ptr_mut
}

impl PtrOfDisparityWLSFilter {
	#[inline] pub fn as_raw_PtrOfDisparityWLSFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDisparityWLSFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::DisparityWLSFilterConst for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_DisparityWLSFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DisparityWLSFilter for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_mut_DisparityWLSFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::DisparityFilterConst for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_DisparityFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::DisparityFilter for PtrOfDisparityWLSFilter {
	#[inline] fn as_raw_mut_DisparityFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


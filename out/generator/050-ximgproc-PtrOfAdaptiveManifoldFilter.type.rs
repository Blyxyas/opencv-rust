pub type PtrOfAdaptiveManifoldFilter = core::Ptr<dyn crate::ximgproc::AdaptiveManifoldFilter>;

ptr_extern! { dyn crate::ximgproc::AdaptiveManifoldFilter,
	cv_PtrOfAdaptiveManifoldFilter_delete, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr, cv_PtrOfAdaptiveManifoldFilter_get_inner_ptr_mut
}

impl PtrOfAdaptiveManifoldFilter {
	#[inline] pub fn as_raw_PtrOfAdaptiveManifoldFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAdaptiveManifoldFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::AdaptiveManifoldFilterConst for PtrOfAdaptiveManifoldFilter {
	#[inline] fn as_raw_AdaptiveManifoldFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::AdaptiveManifoldFilter for PtrOfAdaptiveManifoldFilter {
	#[inline] fn as_raw_mut_AdaptiveManifoldFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAdaptiveManifoldFilter {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAdaptiveManifoldFilter {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


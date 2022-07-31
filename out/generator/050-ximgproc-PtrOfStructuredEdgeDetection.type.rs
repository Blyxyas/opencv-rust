pub type PtrOfStructuredEdgeDetection = core::Ptr<dyn crate::ximgproc::StructuredEdgeDetection>;

ptr_extern! { dyn crate::ximgproc::StructuredEdgeDetection,
	cv_PtrOfStructuredEdgeDetection_delete, cv_PtrOfStructuredEdgeDetection_get_inner_ptr, cv_PtrOfStructuredEdgeDetection_get_inner_ptr_mut
}

impl PtrOfStructuredEdgeDetection {
	#[inline] pub fn as_raw_PtrOfStructuredEdgeDetection(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStructuredEdgeDetection(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::StructuredEdgeDetectionConst for PtrOfStructuredEdgeDetection {
	#[inline] fn as_raw_StructuredEdgeDetection(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::StructuredEdgeDetection for PtrOfStructuredEdgeDetection {
	#[inline] fn as_raw_mut_StructuredEdgeDetection(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStructuredEdgeDetection {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStructuredEdgeDetection {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfSelectiveSearchSegmentation = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentation>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentation,
	cv_PtrOfSelectiveSearchSegmentation_delete, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentation_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentation {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentation(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationConst for PtrOfSelectiveSearchSegmentation {
	#[inline] fn as_raw_SelectiveSearchSegmentation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentation for PtrOfSelectiveSearchSegmentation {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentation {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentation {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


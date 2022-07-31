pub type PtrOfGraphSegmentation = core::Ptr<dyn crate::ximgproc::GraphSegmentation>;

ptr_extern! { dyn crate::ximgproc::GraphSegmentation,
	cv_PtrOfGraphSegmentation_delete, cv_PtrOfGraphSegmentation_get_inner_ptr, cv_PtrOfGraphSegmentation_get_inner_ptr_mut
}

impl PtrOfGraphSegmentation {
	#[inline] pub fn as_raw_PtrOfGraphSegmentation(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGraphSegmentation(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::GraphSegmentationConst for PtrOfGraphSegmentation {
	#[inline] fn as_raw_GraphSegmentation(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::GraphSegmentation for PtrOfGraphSegmentation {
	#[inline] fn as_raw_mut_GraphSegmentation(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGraphSegmentation {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGraphSegmentation {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


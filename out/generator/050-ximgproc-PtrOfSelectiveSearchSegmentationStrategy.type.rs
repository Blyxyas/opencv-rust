pub type PtrOfSelectiveSearchSegmentationStrategy = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategy,
	cv_PtrOfSelectiveSearchSegmentationStrategy_delete, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategy_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategy {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategy {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategy {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategy {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategy {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


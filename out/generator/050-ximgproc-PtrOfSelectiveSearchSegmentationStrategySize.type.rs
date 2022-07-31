pub type PtrOfSelectiveSearchSegmentationStrategySize = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategySize,
	cv_PtrOfSelectiveSearchSegmentationStrategySize_delete, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategySize_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategySizeConst for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategySize(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategySize for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategySize(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategySize {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategySize, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
	cv_PtrOfSelectiveSearchSegmentationStrategySize_to_PtrOfSelectiveSearchSegmentationStrategy,
}


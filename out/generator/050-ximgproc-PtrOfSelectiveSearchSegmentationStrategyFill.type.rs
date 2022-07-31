pub type PtrOfSelectiveSearchSegmentationStrategyFill = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyFill,
	cv_PtrOfSelectiveSearchSegmentationStrategyFill_delete, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyFill_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyFillConst for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyFill(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyFill for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyFill(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyFill {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyFill, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
	cv_PtrOfSelectiveSearchSegmentationStrategyFill_to_PtrOfSelectiveSearchSegmentationStrategy,
}


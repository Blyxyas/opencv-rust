pub type PtrOfSelectiveSearchSegmentationStrategyMultiple = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple,
	cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_delete, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultipleConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyMultiple(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyMultiple for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyMultiple(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyMultiple {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyMultiple, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
	cv_PtrOfSelectiveSearchSegmentationStrategyMultiple_to_PtrOfSelectiveSearchSegmentationStrategy,
}


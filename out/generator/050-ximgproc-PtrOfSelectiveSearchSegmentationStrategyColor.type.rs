pub type PtrOfSelectiveSearchSegmentationStrategyColor = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyColor,
	cv_PtrOfSelectiveSearchSegmentationStrategyColor_delete, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyColor_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyColorConst for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyColor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyColor for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyColor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyColor {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyColor, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
	cv_PtrOfSelectiveSearchSegmentationStrategyColor_to_PtrOfSelectiveSearchSegmentationStrategy,
}


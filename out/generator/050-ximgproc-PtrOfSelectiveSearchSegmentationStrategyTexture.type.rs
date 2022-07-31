pub type PtrOfSelectiveSearchSegmentationStrategyTexture = core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture>;

ptr_extern! { dyn crate::ximgproc::SelectiveSearchSegmentationStrategyTexture,
	cv_PtrOfSelectiveSearchSegmentationStrategyTexture_delete, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr, cv_PtrOfSelectiveSearchSegmentationStrategyTexture_get_inner_ptr_mut
}

impl PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] pub fn as_raw_PtrOfSelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTextureConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategyTexture(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyTexture for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategyTexture(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategyConst for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_SelectiveSearchSegmentationStrategy(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ximgproc::SelectiveSearchSegmentationStrategy for PtrOfSelectiveSearchSegmentationStrategyTexture {
	#[inline] fn as_raw_mut_SelectiveSearchSegmentationStrategy(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfSelectiveSearchSegmentationStrategyTexture, core::Ptr<dyn crate::ximgproc::SelectiveSearchSegmentationStrategy>,
	cv_PtrOfSelectiveSearchSegmentationStrategyTexture_to_PtrOfSelectiveSearchSegmentationStrategy,
}


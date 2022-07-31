pub type PtrOfGrayCodePattern = core::Ptr<dyn crate::structured_light::GrayCodePattern>;

ptr_extern! { dyn crate::structured_light::GrayCodePattern,
	cv_PtrOfGrayCodePattern_delete, cv_PtrOfGrayCodePattern_get_inner_ptr, cv_PtrOfGrayCodePattern_get_inner_ptr_mut
}

impl PtrOfGrayCodePattern {
	#[inline] pub fn as_raw_PtrOfGrayCodePattern(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGrayCodePattern(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::structured_light::GrayCodePatternConst for PtrOfGrayCodePattern {
	#[inline] fn as_raw_GrayCodePattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
	#[inline] fn as_raw_mut_GrayCodePattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfGrayCodePattern {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfGrayCodePattern {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::structured_light::StructuredLightPatternConst for PtrOfGrayCodePattern {
	#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
	#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


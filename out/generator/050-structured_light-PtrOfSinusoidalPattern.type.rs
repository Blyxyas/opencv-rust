pub type PtrOfSinusoidalPattern = core::Ptr<dyn crate::structured_light::SinusoidalPattern>;

ptr_extern! { dyn crate::structured_light::SinusoidalPattern,
	cv_PtrOfSinusoidalPattern_delete, cv_PtrOfSinusoidalPattern_get_inner_ptr, cv_PtrOfSinusoidalPattern_get_inner_ptr_mut
}

impl PtrOfSinusoidalPattern {
	#[inline] pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSinusoidalPattern(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::structured_light::SinusoidalPatternConst for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_SinusoidalPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_mut_SinusoidalPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::structured_light::StructuredLightPatternConst for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_StructuredLightPattern(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
	#[inline] fn as_raw_mut_StructuredLightPattern(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


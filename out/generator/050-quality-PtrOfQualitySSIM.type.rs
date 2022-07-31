pub type PtrOfQualitySSIM = core::Ptr<crate::quality::QualitySSIM>;

ptr_extern! { crate::quality::QualitySSIM,
	cv_PtrOfQualitySSIM_delete, cv_PtrOfQualitySSIM_get_inner_ptr, cv_PtrOfQualitySSIM_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::quality::QualitySSIM, cv_PtrOfQualitySSIM_new }

impl PtrOfQualitySSIM {
	#[inline] pub fn as_raw_PtrOfQualitySSIM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualitySSIM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualitySSIMTraitConst for PtrOfQualitySSIM {
	#[inline] fn as_raw_QualitySSIM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualitySSIMTrait for PtrOfQualitySSIM {
	#[inline] fn as_raw_mut_QualitySSIM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQualitySSIM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQualitySSIM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::quality::QualityBaseConst for PtrOfQualitySSIM {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBase for PtrOfQualitySSIM {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfQualityMSE = core::Ptr<crate::quality::QualityMSE>;

ptr_extern! { crate::quality::QualityMSE,
	cv_PtrOfQualityMSE_delete, cv_PtrOfQualityMSE_get_inner_ptr, cv_PtrOfQualityMSE_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::quality::QualityMSE, cv_PtrOfQualityMSE_new }

impl PtrOfQualityMSE {
	#[inline] pub fn as_raw_PtrOfQualityMSE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityMSE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityMSETraitConst for PtrOfQualityMSE {
	#[inline] fn as_raw_QualityMSE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityMSETrait for PtrOfQualityMSE {
	#[inline] fn as_raw_mut_QualityMSE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQualityMSE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQualityMSE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::quality::QualityBaseConst for PtrOfQualityMSE {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBase for PtrOfQualityMSE {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


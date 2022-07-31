pub type PtrOfQualityBRISQUE = core::Ptr<crate::quality::QualityBRISQUE>;

ptr_extern! { crate::quality::QualityBRISQUE,
	cv_PtrOfQualityBRISQUE_delete, cv_PtrOfQualityBRISQUE_get_inner_ptr, cv_PtrOfQualityBRISQUE_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::quality::QualityBRISQUE, cv_PtrOfQualityBRISQUE_new }

impl PtrOfQualityBRISQUE {
	#[inline] pub fn as_raw_PtrOfQualityBRISQUE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQualityBRISQUE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::quality::QualityBRISQUETraitConst for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_QualityBRISQUE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBRISQUETrait for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_mut_QualityBRISQUE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::quality::QualityBaseConst for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_QualityBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::quality::QualityBase for PtrOfQualityBRISQUE {
	#[inline] fn as_raw_mut_QualityBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


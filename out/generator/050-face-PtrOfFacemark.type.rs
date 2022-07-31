pub type PtrOfFacemark = core::Ptr<dyn crate::face::Facemark>;

ptr_extern! { dyn crate::face::Facemark,
	cv_PtrOfFacemark_delete, cv_PtrOfFacemark_get_inner_ptr, cv_PtrOfFacemark_get_inner_ptr_mut
}

impl PtrOfFacemark {
	#[inline] pub fn as_raw_PtrOfFacemark(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFacemark(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::FacemarkConst for PtrOfFacemark {
	#[inline] fn as_raw_Facemark(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::Facemark for PtrOfFacemark {
	#[inline] fn as_raw_mut_Facemark(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFacemark {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFacemark {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


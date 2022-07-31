pub type PtrOfBIF = core::Ptr<dyn crate::face::BIF>;

ptr_extern! { dyn crate::face::BIF,
	cv_PtrOfBIF_delete, cv_PtrOfBIF_get_inner_ptr, cv_PtrOfBIF_get_inner_ptr_mut
}

impl PtrOfBIF {
	#[inline] pub fn as_raw_PtrOfBIF(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBIF(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::BIFConst for PtrOfBIF {
	#[inline] fn as_raw_BIF(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::BIF for PtrOfBIF {
	#[inline] fn as_raw_mut_BIF(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBIF {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBIF {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfMACE = core::Ptr<dyn crate::face::MACE>;

ptr_extern! { dyn crate::face::MACE,
	cv_PtrOfMACE_delete, cv_PtrOfMACE_get_inner_ptr, cv_PtrOfMACE_get_inner_ptr_mut
}

impl PtrOfMACE {
	#[inline] pub fn as_raw_PtrOfMACE(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMACE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::face::MACEConst for PtrOfMACE {
	#[inline] fn as_raw_MACE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::MACE for PtrOfMACE {
	#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMACE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMACE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


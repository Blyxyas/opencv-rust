pub type PtrOfMCC_CChecker = core::Ptr<dyn crate::mcc::MCC_CChecker>;

ptr_extern! { dyn crate::mcc::MCC_CChecker,
	cv_PtrOfMCC_CChecker_delete, cv_PtrOfMCC_CChecker_get_inner_ptr, cv_PtrOfMCC_CChecker_get_inner_ptr_mut
}

impl PtrOfMCC_CChecker {
	#[inline] pub fn as_raw_PtrOfMCC_CChecker(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CChecker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerConst for PtrOfMCC_CChecker {
	#[inline] fn as_raw_MCC_CChecker(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CChecker for PtrOfMCC_CChecker {
	#[inline] fn as_raw_mut_MCC_CChecker(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


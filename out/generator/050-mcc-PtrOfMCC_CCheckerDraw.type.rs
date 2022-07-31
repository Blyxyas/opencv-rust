pub type PtrOfMCC_CCheckerDraw = core::Ptr<dyn crate::mcc::MCC_CCheckerDraw>;

ptr_extern! { dyn crate::mcc::MCC_CCheckerDraw,
	cv_PtrOfMCC_CCheckerDraw_delete, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr, cv_PtrOfMCC_CCheckerDraw_get_inner_ptr_mut
}

impl PtrOfMCC_CCheckerDraw {
	#[inline] pub fn as_raw_PtrOfMCC_CCheckerDraw(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_CCheckerDraw(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::mcc::MCC_CCheckerDrawConst for PtrOfMCC_CCheckerDraw {
	#[inline] fn as_raw_MCC_CCheckerDraw(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_CCheckerDraw for PtrOfMCC_CCheckerDraw {
	#[inline] fn as_raw_mut_MCC_CCheckerDraw(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


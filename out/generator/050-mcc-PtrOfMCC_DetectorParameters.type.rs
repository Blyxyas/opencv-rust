pub type PtrOfMCC_DetectorParameters = core::Ptr<crate::mcc::MCC_DetectorParameters>;

ptr_extern! { crate::mcc::MCC_DetectorParameters,
	cv_PtrOfMCC_DetectorParameters_delete, cv_PtrOfMCC_DetectorParameters_get_inner_ptr, cv_PtrOfMCC_DetectorParameters_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::mcc::MCC_DetectorParameters, cv_PtrOfMCC_DetectorParameters_new }

impl PtrOfMCC_DetectorParameters {
	#[inline] pub fn as_raw_PtrOfMCC_DetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMCC_DetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::mcc::MCC_DetectorParametersTraitConst for PtrOfMCC_DetectorParameters {
	#[inline] fn as_raw_MCC_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::mcc::MCC_DetectorParametersTrait for PtrOfMCC_DetectorParameters {
	#[inline] fn as_raw_mut_MCC_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


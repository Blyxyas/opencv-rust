pub type PtrOfEstimateParameters = core::Ptr<crate::aruco::EstimateParameters>;

ptr_extern! { crate::aruco::EstimateParameters,
	cv_PtrOfEstimateParameters_delete, cv_PtrOfEstimateParameters_get_inner_ptr, cv_PtrOfEstimateParameters_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::EstimateParameters, cv_PtrOfEstimateParameters_new }

impl PtrOfEstimateParameters {
	#[inline] pub fn as_raw_PtrOfEstimateParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfEstimateParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::EstimateParametersTraitConst for PtrOfEstimateParameters {
	#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::EstimateParametersTrait for PtrOfEstimateParameters {
	#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


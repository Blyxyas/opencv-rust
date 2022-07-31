pub type PtrOfDetectorParameters = core::Ptr<crate::aruco::DetectorParameters>;

ptr_extern! { crate::aruco::DetectorParameters,
	cv_PtrOfDetectorParameters_delete, cv_PtrOfDetectorParameters_get_inner_ptr, cv_PtrOfDetectorParameters_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::aruco::DetectorParameters, cv_PtrOfDetectorParameters_new }

impl PtrOfDetectorParameters {
	#[inline] pub fn as_raw_PtrOfDetectorParameters(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetectorParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::aruco::DetectorParametersTraitConst for PtrOfDetectorParameters {
	#[inline] fn as_raw_DetectorParameters(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::aruco::DetectorParametersTrait for PtrOfDetectorParameters {
	#[inline] fn as_raw_mut_DetectorParameters(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


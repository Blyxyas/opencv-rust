pub type PtrOfDetail_ExposureCompensator = core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>;

ptr_extern! { dyn crate::stitching::Detail_ExposureCompensator,
	cv_PtrOfDetail_ExposureCompensator_delete, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr, cv_PtrOfDetail_ExposureCompensator_get_inner_ptr_mut
}

impl PtrOfDetail_ExposureCompensator {
	#[inline] pub fn as_raw_PtrOfDetail_ExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_ExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_ExposureCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ExposureCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfDetail_BlocksCompensator = core::Ptr<dyn crate::stitching::Detail_BlocksCompensator>;

ptr_extern! { dyn crate::stitching::Detail_BlocksCompensator,
	cv_PtrOfDetail_BlocksCompensator_delete, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr, cv_PtrOfDetail_BlocksCompensator_get_inner_ptr_mut
}

impl PtrOfDetail_BlocksCompensator {
	#[inline] pub fn as_raw_PtrOfDetail_BlocksCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BlocksCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_BlocksCompensatorConst for PtrOfDetail_BlocksCompensator {
	#[inline] fn as_raw_Detail_BlocksCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BlocksCompensator for PtrOfDetail_BlocksCompensator {
	#[inline] fn as_raw_mut_Detail_BlocksCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_BlocksCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_BlocksCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BlocksCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
	cv_PtrOfDetail_BlocksCompensator_to_PtrOfDetail_ExposureCompensator,
}


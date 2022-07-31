pub type PtrOfDetail_NoExposureCompensator = core::Ptr<crate::stitching::Detail_NoExposureCompensator>;

ptr_extern! { crate::stitching::Detail_NoExposureCompensator,
	cv_PtrOfDetail_NoExposureCompensator_delete, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr, cv_PtrOfDetail_NoExposureCompensator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_NoExposureCompensator, cv_PtrOfDetail_NoExposureCompensator_new }

impl PtrOfDetail_NoExposureCompensator {
	#[inline] pub fn as_raw_PtrOfDetail_NoExposureCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_NoExposureCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_NoExposureCompensatorTraitConst for PtrOfDetail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_NoExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_NoExposureCompensatorTrait for PtrOfDetail_NoExposureCompensator {
	#[inline] fn as_raw_mut_Detail_NoExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_NoExposureCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_NoExposureCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_NoExposureCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
	cv_PtrOfDetail_NoExposureCompensator_to_PtrOfDetail_ExposureCompensator,
}


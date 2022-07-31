pub type PtrOfDetail_GainCompensator = core::Ptr<crate::stitching::Detail_GainCompensator>;

ptr_extern! { crate::stitching::Detail_GainCompensator,
	cv_PtrOfDetail_GainCompensator_delete, cv_PtrOfDetail_GainCompensator_get_inner_ptr, cv_PtrOfDetail_GainCompensator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_GainCompensator, cv_PtrOfDetail_GainCompensator_new }

impl PtrOfDetail_GainCompensator {
	#[inline] pub fn as_raw_PtrOfDetail_GainCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_GainCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_GainCompensatorTraitConst for PtrOfDetail_GainCompensator {
	#[inline] fn as_raw_Detail_GainCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_GainCompensatorTrait for PtrOfDetail_GainCompensator {
	#[inline] fn as_raw_mut_Detail_GainCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_GainCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_GainCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_GainCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
	cv_PtrOfDetail_GainCompensator_to_PtrOfDetail_ExposureCompensator,
}


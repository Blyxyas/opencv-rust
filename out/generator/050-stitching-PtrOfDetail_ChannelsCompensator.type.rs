pub type PtrOfDetail_ChannelsCompensator = core::Ptr<crate::stitching::Detail_ChannelsCompensator>;

ptr_extern! { crate::stitching::Detail_ChannelsCompensator,
	cv_PtrOfDetail_ChannelsCompensator_delete, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr, cv_PtrOfDetail_ChannelsCompensator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::stitching::Detail_ChannelsCompensator, cv_PtrOfDetail_ChannelsCompensator_new }

impl PtrOfDetail_ChannelsCompensator {
	#[inline] pub fn as_raw_PtrOfDetail_ChannelsCompensator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_ChannelsCompensator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::stitching::Detail_ChannelsCompensatorTraitConst for PtrOfDetail_ChannelsCompensator {
	#[inline] fn as_raw_Detail_ChannelsCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ChannelsCompensatorTrait for PtrOfDetail_ChannelsCompensator {
	#[inline] fn as_raw_mut_Detail_ChannelsCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_ExposureCompensatorConst for PtrOfDetail_ChannelsCompensator {
	#[inline] fn as_raw_Detail_ExposureCompensator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_ExposureCompensator for PtrOfDetail_ChannelsCompensator {
	#[inline] fn as_raw_mut_Detail_ExposureCompensator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_ChannelsCompensator, core::Ptr<dyn crate::stitching::Detail_ExposureCompensator>,
	cv_PtrOfDetail_ChannelsCompensator_to_PtrOfDetail_ExposureCompensator,
}


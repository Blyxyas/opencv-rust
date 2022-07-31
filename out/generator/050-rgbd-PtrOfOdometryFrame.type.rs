pub type PtrOfOdometryFrame = core::Ptr<crate::rgbd::OdometryFrame>;

ptr_extern! { crate::rgbd::OdometryFrame,
	cv_PtrOfOdometryFrame_delete, cv_PtrOfOdometryFrame_get_inner_ptr, cv_PtrOfOdometryFrame_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::OdometryFrame, cv_PtrOfOdometryFrame_new }

impl PtrOfOdometryFrame {
	#[inline] pub fn as_raw_PtrOfOdometryFrame(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOdometryFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryFrameTraitConst for PtrOfOdometryFrame {
	#[inline] fn as_raw_OdometryFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::OdometryFrameTrait for PtrOfOdometryFrame {
	#[inline] fn as_raw_mut_OdometryFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::RgbdFrameTraitConst for PtrOfOdometryFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for PtrOfOdometryFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


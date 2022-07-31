pub type PtrOfRgbdFrame = core::Ptr<crate::rgbd::RgbdFrame>;

ptr_extern! { crate::rgbd::RgbdFrame,
	cv_PtrOfRgbdFrame_delete, cv_PtrOfRgbdFrame_get_inner_ptr, cv_PtrOfRgbdFrame_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::RgbdFrame, cv_PtrOfRgbdFrame_new }

impl PtrOfRgbdFrame {
	#[inline] pub fn as_raw_PtrOfRgbdFrame(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdFrame(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdFrameTraitConst for PtrOfRgbdFrame {
	#[inline] fn as_raw_RgbdFrame(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdFrameTrait for PtrOfRgbdFrame {
	#[inline] fn as_raw_mut_RgbdFrame(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


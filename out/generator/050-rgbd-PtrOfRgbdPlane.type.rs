pub type PtrOfRgbdPlane = core::Ptr<crate::rgbd::RgbdPlane>;

ptr_extern! { crate::rgbd::RgbdPlane,
	cv_PtrOfRgbdPlane_delete, cv_PtrOfRgbdPlane_get_inner_ptr, cv_PtrOfRgbdPlane_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::RgbdPlane, cv_PtrOfRgbdPlane_new }

impl PtrOfRgbdPlane {
	#[inline] pub fn as_raw_PtrOfRgbdPlane(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdPlane(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdPlaneTraitConst for PtrOfRgbdPlane {
	#[inline] fn as_raw_RgbdPlane(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdPlaneTrait for PtrOfRgbdPlane {
	#[inline] fn as_raw_mut_RgbdPlane(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRgbdPlane {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRgbdPlane {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


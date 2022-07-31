pub type PtrOfRgbdOdometry = core::Ptr<crate::rgbd::RgbdOdometry>;

ptr_extern! { crate::rgbd::RgbdOdometry,
	cv_PtrOfRgbdOdometry_delete, cv_PtrOfRgbdOdometry_get_inner_ptr, cv_PtrOfRgbdOdometry_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::RgbdOdometry, cv_PtrOfRgbdOdometry_new }

impl PtrOfRgbdOdometry {
	#[inline] pub fn as_raw_PtrOfRgbdOdometry(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdOdometryTraitConst for PtrOfRgbdOdometry {
	#[inline] fn as_raw_RgbdOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdOdometryTrait for PtrOfRgbdOdometry {
	#[inline] fn as_raw_mut_RgbdOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRgbdOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRgbdOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::OdometryConst for PtrOfRgbdOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Odometry for PtrOfRgbdOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


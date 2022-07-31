pub type PtrOfRgbdICPOdometry = core::Ptr<crate::rgbd::RgbdICPOdometry>;

ptr_extern! { crate::rgbd::RgbdICPOdometry,
	cv_PtrOfRgbdICPOdometry_delete, cv_PtrOfRgbdICPOdometry_get_inner_ptr, cv_PtrOfRgbdICPOdometry_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::RgbdICPOdometry, cv_PtrOfRgbdICPOdometry_new }

impl PtrOfRgbdICPOdometry {
	#[inline] pub fn as_raw_PtrOfRgbdICPOdometry(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRgbdICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::RgbdICPOdometryTraitConst for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_RgbdICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::RgbdICPOdometryTrait for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_mut_RgbdICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::OdometryConst for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Odometry for PtrOfRgbdICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


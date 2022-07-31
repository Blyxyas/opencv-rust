pub type PtrOfOdometry = core::Ptr<dyn crate::rgbd::Odometry>;

ptr_extern! { dyn crate::rgbd::Odometry,
	cv_PtrOfOdometry_delete, cv_PtrOfOdometry_get_inner_ptr, cv_PtrOfOdometry_get_inner_ptr_mut
}

impl PtrOfOdometry {
	#[inline] pub fn as_raw_PtrOfOdometry(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::OdometryConst for PtrOfOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Odometry for PtrOfOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


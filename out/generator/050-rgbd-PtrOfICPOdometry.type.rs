pub type PtrOfICPOdometry = core::Ptr<crate::rgbd::ICPOdometry>;

ptr_extern! { crate::rgbd::ICPOdometry,
	cv_PtrOfICPOdometry_delete, cv_PtrOfICPOdometry_get_inner_ptr, cv_PtrOfICPOdometry_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::ICPOdometry, cv_PtrOfICPOdometry_new }

impl PtrOfICPOdometry {
	#[inline] pub fn as_raw_PtrOfICPOdometry(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::ICPOdometryTraitConst for PtrOfICPOdometry {
	#[inline] fn as_raw_ICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::ICPOdometryTrait for PtrOfICPOdometry {
	#[inline] fn as_raw_mut_ICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::OdometryConst for PtrOfICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Odometry for PtrOfICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


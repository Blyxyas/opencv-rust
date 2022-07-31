pub type PtrOfFastICPOdometry = core::Ptr<crate::rgbd::FastICPOdometry>;

ptr_extern! { crate::rgbd::FastICPOdometry,
	cv_PtrOfFastICPOdometry_delete, cv_PtrOfFastICPOdometry_get_inner_ptr, cv_PtrOfFastICPOdometry_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::FastICPOdometry, cv_PtrOfFastICPOdometry_new }

impl PtrOfFastICPOdometry {
	#[inline] pub fn as_raw_PtrOfFastICPOdometry(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFastICPOdometry(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::FastICPOdometryTraitConst for PtrOfFastICPOdometry {
	#[inline] fn as_raw_FastICPOdometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::FastICPOdometryTrait for PtrOfFastICPOdometry {
	#[inline] fn as_raw_mut_FastICPOdometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfFastICPOdometry {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfFastICPOdometry {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::OdometryConst for PtrOfFastICPOdometry {
	#[inline] fn as_raw_Odometry(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Odometry for PtrOfFastICPOdometry {
	#[inline] fn as_raw_mut_Odometry(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


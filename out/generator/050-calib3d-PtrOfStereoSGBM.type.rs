pub type PtrOfStereoSGBM = core::Ptr<dyn crate::calib3d::StereoSGBM>;

ptr_extern! { dyn crate::calib3d::StereoSGBM,
	cv_PtrOfStereoSGBM_delete, cv_PtrOfStereoSGBM_get_inner_ptr, cv_PtrOfStereoSGBM_get_inner_ptr_mut
}

impl PtrOfStereoSGBM {
	#[inline] pub fn as_raw_PtrOfStereoSGBM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereoSGBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoSGBMConst for PtrOfStereoSGBM {
	#[inline] fn as_raw_StereoSGBM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
	#[inline] fn as_raw_mut_StereoSGBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStereoSGBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStereoSGBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::calib3d::StereoMatcherConst for PtrOfStereoSGBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


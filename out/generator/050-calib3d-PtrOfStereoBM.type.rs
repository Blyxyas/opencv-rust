pub type PtrOfStereoBM = core::Ptr<dyn crate::calib3d::StereoBM>;

ptr_extern! { dyn crate::calib3d::StereoBM,
	cv_PtrOfStereoBM_delete, cv_PtrOfStereoBM_get_inner_ptr, cv_PtrOfStereoBM_get_inner_ptr_mut
}

impl PtrOfStereoBM {
	#[inline] pub fn as_raw_PtrOfStereoBM(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStereoBM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::calib3d::StereoBMConst for PtrOfStereoBM {
	#[inline] fn as_raw_StereoBM(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoBM for PtrOfStereoBM {
	#[inline] fn as_raw_mut_StereoBM(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStereoBM {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStereoBM {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::calib3d::StereoMatcherConst for PtrOfStereoBM {
	#[inline] fn as_raw_StereoMatcher(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
	#[inline] fn as_raw_mut_StereoMatcher(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


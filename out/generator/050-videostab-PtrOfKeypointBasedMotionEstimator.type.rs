pub type PtrOfKeypointBasedMotionEstimator = core::Ptr<crate::videostab::KeypointBasedMotionEstimator>;

ptr_extern! { crate::videostab::KeypointBasedMotionEstimator,
	cv_PtrOfKeypointBasedMotionEstimator_delete, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr, cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::KeypointBasedMotionEstimator, cv_PtrOfKeypointBasedMotionEstimator_new }

impl PtrOfKeypointBasedMotionEstimator {
	#[inline] pub fn as_raw_PtrOfKeypointBasedMotionEstimator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTraitConst for PtrOfKeypointBasedMotionEstimator {
	#[inline] fn as_raw_KeypointBasedMotionEstimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::KeypointBasedMotionEstimatorTrait for PtrOfKeypointBasedMotionEstimator {
	#[inline] fn as_raw_mut_KeypointBasedMotionEstimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfKeypointBasedMotionEstimator {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBase for PtrOfKeypointBasedMotionEstimator {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfKeypointBasedMotionEstimator, core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>,
	cv_PtrOfKeypointBasedMotionEstimator_to_PtrOfImageMotionEstimatorBase,
}


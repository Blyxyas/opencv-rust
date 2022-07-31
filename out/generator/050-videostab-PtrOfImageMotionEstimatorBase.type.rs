pub type PtrOfImageMotionEstimatorBase = core::Ptr<dyn crate::videostab::ImageMotionEstimatorBase>;

ptr_extern! { dyn crate::videostab::ImageMotionEstimatorBase,
	cv_PtrOfImageMotionEstimatorBase_delete, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr, cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut
}

impl PtrOfImageMotionEstimatorBase {
	#[inline] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfImageMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ImageMotionEstimatorBaseConst for PtrOfImageMotionEstimatorBase {
	#[inline] fn as_raw_ImageMotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
	#[inline] fn as_raw_mut_ImageMotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


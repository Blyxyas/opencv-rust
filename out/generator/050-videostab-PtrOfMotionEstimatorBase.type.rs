pub type PtrOfMotionEstimatorBase = core::Ptr<dyn crate::videostab::MotionEstimatorBase>;

ptr_extern! { dyn crate::videostab::MotionEstimatorBase,
	cv_PtrOfMotionEstimatorBase_delete, cv_PtrOfMotionEstimatorBase_get_inner_ptr, cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut
}

impl PtrOfMotionEstimatorBase {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorBase {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


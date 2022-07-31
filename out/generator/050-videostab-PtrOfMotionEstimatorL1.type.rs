pub type PtrOfMotionEstimatorL1 = core::Ptr<crate::videostab::MotionEstimatorL1>;

ptr_extern! { crate::videostab::MotionEstimatorL1,
	cv_PtrOfMotionEstimatorL1_delete, cv_PtrOfMotionEstimatorL1_get_inner_ptr, cv_PtrOfMotionEstimatorL1_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::MotionEstimatorL1, cv_PtrOfMotionEstimatorL1_new }

impl PtrOfMotionEstimatorL1 {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorL1(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorL1(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorL1TraitConst for PtrOfMotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorL1(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorL1Trait for PtrOfMotionEstimatorL1 {
	#[inline] fn as_raw_mut_MotionEstimatorL1(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorL1 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorL1 {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMotionEstimatorL1, core::Ptr<dyn crate::videostab::MotionEstimatorBase>,
	cv_PtrOfMotionEstimatorL1_to_PtrOfMotionEstimatorBase,
}


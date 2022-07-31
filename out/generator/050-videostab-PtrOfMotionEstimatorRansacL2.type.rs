pub type PtrOfMotionEstimatorRansacL2 = core::Ptr<crate::videostab::MotionEstimatorRansacL2>;

ptr_extern! { crate::videostab::MotionEstimatorRansacL2,
	cv_PtrOfMotionEstimatorRansacL2_delete, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr, cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::MotionEstimatorRansacL2, cv_PtrOfMotionEstimatorRansacL2_new }

impl PtrOfMotionEstimatorRansacL2 {
	#[inline] pub fn as_raw_PtrOfMotionEstimatorRansacL2(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionEstimatorRansacL2(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionEstimatorRansacL2TraitConst for PtrOfMotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorRansacL2(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorRansacL2Trait for PtrOfMotionEstimatorRansacL2 {
	#[inline] fn as_raw_mut_MotionEstimatorRansacL2(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::MotionEstimatorBaseConst for PtrOfMotionEstimatorRansacL2 {
	#[inline] fn as_raw_MotionEstimatorBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorRansacL2 {
	#[inline] fn as_raw_mut_MotionEstimatorBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMotionEstimatorRansacL2, core::Ptr<dyn crate::videostab::MotionEstimatorBase>,
	cv_PtrOfMotionEstimatorRansacL2_to_PtrOfMotionEstimatorBase,
}


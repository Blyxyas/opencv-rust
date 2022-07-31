pub type PtrOfMotionFilterBase = core::Ptr<dyn crate::videostab::MotionFilterBase>;

ptr_extern! { dyn crate::videostab::MotionFilterBase,
	cv_PtrOfMotionFilterBase_delete, cv_PtrOfMotionFilterBase_get_inner_ptr, cv_PtrOfMotionFilterBase_get_inner_ptr_mut
}

impl PtrOfMotionFilterBase {
	#[inline] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionFilterBase(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionFilterBaseConst for PtrOfMotionFilterBase {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerConst for PtrOfMotionFilterBase {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMotionFilterBase, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
	cv_PtrOfMotionFilterBase_to_PtrOfIMotionStabilizer,
}


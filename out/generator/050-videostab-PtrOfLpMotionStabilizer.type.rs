pub type PtrOfLpMotionStabilizer = core::Ptr<crate::videostab::LpMotionStabilizer>;

ptr_extern! { crate::videostab::LpMotionStabilizer,
	cv_PtrOfLpMotionStabilizer_delete, cv_PtrOfLpMotionStabilizer_get_inner_ptr, cv_PtrOfLpMotionStabilizer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::LpMotionStabilizer, cv_PtrOfLpMotionStabilizer_new }

impl PtrOfLpMotionStabilizer {
	#[inline] pub fn as_raw_PtrOfLpMotionStabilizer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLpMotionStabilizer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::LpMotionStabilizerTraitConst for PtrOfLpMotionStabilizer {
	#[inline] fn as_raw_LpMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::LpMotionStabilizerTrait for PtrOfLpMotionStabilizer {
	#[inline] fn as_raw_mut_LpMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerConst for PtrOfLpMotionStabilizer {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizer for PtrOfLpMotionStabilizer {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfLpMotionStabilizer, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
	cv_PtrOfLpMotionStabilizer_to_PtrOfIMotionStabilizer,
}


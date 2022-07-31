pub type PtrOfGaussianMotionFilter = core::Ptr<crate::videostab::GaussianMotionFilter>;

ptr_extern! { crate::videostab::GaussianMotionFilter,
	cv_PtrOfGaussianMotionFilter_delete, cv_PtrOfGaussianMotionFilter_get_inner_ptr, cv_PtrOfGaussianMotionFilter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::GaussianMotionFilter, cv_PtrOfGaussianMotionFilter_new }

impl PtrOfGaussianMotionFilter {
	#[inline] pub fn as_raw_PtrOfGaussianMotionFilter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGaussianMotionFilter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::GaussianMotionFilterTraitConst for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_GaussianMotionFilter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::GaussianMotionFilterTrait for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_mut_GaussianMotionFilter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerConst for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizer for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
	cv_PtrOfGaussianMotionFilter_to_PtrOfIMotionStabilizer,
}

impl crate::videostab::MotionFilterBaseConst for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_MotionFilterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionFilterBase for PtrOfGaussianMotionFilter {
	#[inline] fn as_raw_mut_MotionFilterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfGaussianMotionFilter, core::Ptr<dyn crate::videostab::MotionFilterBase>,
	cv_PtrOfGaussianMotionFilter_to_PtrOfMotionFilterBase,
}


pub type PtrOfMotionStabilizationPipeline = core::Ptr<crate::videostab::MotionStabilizationPipeline>;

ptr_extern! { crate::videostab::MotionStabilizationPipeline,
	cv_PtrOfMotionStabilizationPipeline_delete, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr, cv_PtrOfMotionStabilizationPipeline_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::MotionStabilizationPipeline, cv_PtrOfMotionStabilizationPipeline_new }

impl PtrOfMotionStabilizationPipeline {
	#[inline] pub fn as_raw_PtrOfMotionStabilizationPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionStabilizationPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionStabilizationPipelineTraitConst for PtrOfMotionStabilizationPipeline {
	#[inline] fn as_raw_MotionStabilizationPipeline(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionStabilizationPipelineTrait for PtrOfMotionStabilizationPipeline {
	#[inline] fn as_raw_mut_MotionStabilizationPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::IMotionStabilizerConst for PtrOfMotionStabilizationPipeline {
	#[inline] fn as_raw_IMotionStabilizer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::IMotionStabilizer for PtrOfMotionStabilizationPipeline {
	#[inline] fn as_raw_mut_IMotionStabilizer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMotionStabilizationPipeline, core::Ptr<dyn crate::videostab::IMotionStabilizer>,
	cv_PtrOfMotionStabilizationPipeline_to_PtrOfIMotionStabilizer,
}


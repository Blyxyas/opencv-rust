pub type PtrOfInpaintingPipeline = core::Ptr<crate::videostab::InpaintingPipeline>;

ptr_extern! { crate::videostab::InpaintingPipeline,
	cv_PtrOfInpaintingPipeline_delete, cv_PtrOfInpaintingPipeline_get_inner_ptr, cv_PtrOfInpaintingPipeline_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::InpaintingPipeline, cv_PtrOfInpaintingPipeline_new }

impl PtrOfInpaintingPipeline {
	#[inline] pub fn as_raw_PtrOfInpaintingPipeline(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfInpaintingPipeline(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::InpaintingPipelineTraitConst for PtrOfInpaintingPipeline {
	#[inline] fn as_raw_InpaintingPipeline(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpaintingPipelineTrait for PtrOfInpaintingPipeline {
	#[inline] fn as_raw_mut_InpaintingPipeline(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfInpaintingPipeline {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfInpaintingPipeline {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfInpaintingPipeline, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfInpaintingPipeline_to_PtrOfInpainterBase,
}


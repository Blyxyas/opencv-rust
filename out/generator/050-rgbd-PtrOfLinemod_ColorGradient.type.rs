pub type PtrOfLinemod_ColorGradient = core::Ptr<crate::rgbd::Linemod_ColorGradient>;

ptr_extern! { crate::rgbd::Linemod_ColorGradient,
	cv_PtrOfLinemod_ColorGradient_delete, cv_PtrOfLinemod_ColorGradient_get_inner_ptr, cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Linemod_ColorGradient, cv_PtrOfLinemod_ColorGradient_new }

impl PtrOfLinemod_ColorGradient {
	#[inline] pub fn as_raw_PtrOfLinemod_ColorGradient(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinemod_ColorGradient(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_ColorGradientTraitConst for PtrOfLinemod_ColorGradient {
	#[inline] fn as_raw_Linemod_ColorGradient(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_ColorGradientTrait for PtrOfLinemod_ColorGradient {
	#[inline] fn as_raw_mut_Linemod_ColorGradient(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_ColorGradient {
	#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_Modality for PtrOfLinemod_ColorGradient {
	#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfLinemod_DepthNormal = core::Ptr<crate::rgbd::Linemod_DepthNormal>;

ptr_extern! { crate::rgbd::Linemod_DepthNormal,
	cv_PtrOfLinemod_DepthNormal_delete, cv_PtrOfLinemod_DepthNormal_get_inner_ptr, cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::rgbd::Linemod_DepthNormal, cv_PtrOfLinemod_DepthNormal_new }

impl PtrOfLinemod_DepthNormal {
	#[inline] pub fn as_raw_PtrOfLinemod_DepthNormal(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinemod_DepthNormal(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_DepthNormalTraitConst for PtrOfLinemod_DepthNormal {
	#[inline] fn as_raw_Linemod_DepthNormal(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_DepthNormalTrait for PtrOfLinemod_DepthNormal {
	#[inline] fn as_raw_mut_Linemod_DepthNormal(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_DepthNormal {
	#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_Modality for PtrOfLinemod_DepthNormal {
	#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


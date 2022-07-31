pub type PtrOfLinemod_Modality = core::Ptr<dyn crate::rgbd::Linemod_Modality>;

ptr_extern! { dyn crate::rgbd::Linemod_Modality,
	cv_PtrOfLinemod_Modality_delete, cv_PtrOfLinemod_Modality_get_inner_ptr, cv_PtrOfLinemod_Modality_get_inner_ptr_mut
}

impl PtrOfLinemod_Modality {
	#[inline] pub fn as_raw_PtrOfLinemod_Modality(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLinemod_Modality(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rgbd::Linemod_ModalityConst for PtrOfLinemod_Modality {
	#[inline] fn as_raw_Linemod_Modality(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::rgbd::Linemod_Modality for PtrOfLinemod_Modality {
	#[inline] fn as_raw_mut_Linemod_Modality(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


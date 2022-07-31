pub type PtrOfRetinaFastToneMapping = core::Ptr<dyn crate::bioinspired::RetinaFastToneMapping>;

ptr_extern! { dyn crate::bioinspired::RetinaFastToneMapping,
	cv_PtrOfRetinaFastToneMapping_delete, cv_PtrOfRetinaFastToneMapping_get_inner_ptr, cv_PtrOfRetinaFastToneMapping_get_inner_ptr_mut
}

impl PtrOfRetinaFastToneMapping {
	#[inline] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRetinaFastToneMapping(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bioinspired::RetinaFastToneMappingConst for PtrOfRetinaFastToneMapping {
	#[inline] fn as_raw_RetinaFastToneMapping(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
	#[inline] fn as_raw_mut_RetinaFastToneMapping(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRetinaFastToneMapping {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


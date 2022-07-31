pub type PtrOfRetina = core::Ptr<dyn crate::bioinspired::Retina>;

ptr_extern! { dyn crate::bioinspired::Retina,
	cv_PtrOfRetina_delete, cv_PtrOfRetina_get_inner_ptr, cv_PtrOfRetina_get_inner_ptr_mut
}

impl PtrOfRetina {
	#[inline] pub fn as_raw_PtrOfRetina(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfRetina(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::bioinspired::RetinaConst for PtrOfRetina {
	#[inline] fn as_raw_Retina(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::bioinspired::Retina for PtrOfRetina {
	#[inline] fn as_raw_mut_Retina(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfRetina {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfRetina {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


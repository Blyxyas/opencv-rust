pub type PtrOfStaticSaliencyFineGrained = core::Ptr<crate::saliency::StaticSaliencyFineGrained>;

ptr_extern! { crate::saliency::StaticSaliencyFineGrained,
	cv_PtrOfStaticSaliencyFineGrained_delete, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr, cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::saliency::StaticSaliencyFineGrained, cv_PtrOfStaticSaliencyFineGrained_new }

impl PtrOfStaticSaliencyFineGrained {
	#[inline] pub fn as_raw_PtrOfStaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyFineGrainedTraitConst for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencyFineGrainedTrait for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::SaliencyConst for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::Saliency for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliency for PtrOfStaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


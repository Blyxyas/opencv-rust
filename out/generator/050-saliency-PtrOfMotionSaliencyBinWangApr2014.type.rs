pub type PtrOfMotionSaliencyBinWangApr2014 = core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>;

ptr_extern! { crate::saliency::MotionSaliencyBinWangApr2014,
	cv_PtrOfMotionSaliencyBinWangApr2014_delete, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr, cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::saliency::MotionSaliencyBinWangApr2014, cv_PtrOfMotionSaliencyBinWangApr2014_new }

impl PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] pub fn as_raw_PtrOfMotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014Trait for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::MotionSaliencyConst for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::MotionSaliency for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::SaliencyConst for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::Saliency for PtrOfMotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


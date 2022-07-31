pub type PtrOfStaticSaliencySpectralResidual = core::Ptr<crate::saliency::StaticSaliencySpectralResidual>;

ptr_extern! { crate::saliency::StaticSaliencySpectralResidual,
	cv_PtrOfStaticSaliencySpectralResidual_delete, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr, cv_PtrOfStaticSaliencySpectralResidual_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::saliency::StaticSaliencySpectralResidual, cv_PtrOfStaticSaliencySpectralResidual_new }

impl PtrOfStaticSaliencySpectralResidual {
	#[inline] pub fn as_raw_PtrOfStaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencySpectralResidualTraitConst for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencySpectralResidualTrait for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::SaliencyConst for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::Saliency for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliency for PtrOfStaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfOCRBeamSearchDecoder_ClassifierCallback = core::Ptr<crate::text::OCRBeamSearchDecoder_ClassifierCallback>;

ptr_extern! { crate::text::OCRBeamSearchDecoder_ClassifierCallback,
	cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder_ClassifierCallback, cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_new }

impl PtrOfOCRBeamSearchDecoder_ClassifierCallback {
	#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTraitConst for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
	#[inline] fn as_raw_OCRBeamSearchDecoder_ClassifierCallback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRBeamSearchDecoder_ClassifierCallbackTrait for PtrOfOCRBeamSearchDecoder_ClassifierCallback {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder_ClassifierCallback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


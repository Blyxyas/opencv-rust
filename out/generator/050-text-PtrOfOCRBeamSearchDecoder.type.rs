pub type PtrOfOCRBeamSearchDecoder = core::Ptr<crate::text::OCRBeamSearchDecoder>;

ptr_extern! { crate::text::OCRBeamSearchDecoder,
	cv_PtrOfOCRBeamSearchDecoder_delete, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr, cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::text::OCRBeamSearchDecoder, cv_PtrOfOCRBeamSearchDecoder_new }

impl PtrOfOCRBeamSearchDecoder {
	#[inline] pub fn as_raw_PtrOfOCRBeamSearchDecoder(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfOCRBeamSearchDecoder(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::OCRBeamSearchDecoderTraitConst for PtrOfOCRBeamSearchDecoder {
	#[inline] fn as_raw_OCRBeamSearchDecoder(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::OCRBeamSearchDecoderTrait for PtrOfOCRBeamSearchDecoder {
	#[inline] fn as_raw_mut_OCRBeamSearchDecoder(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::text::BaseOCRConst for PtrOfOCRBeamSearchDecoder {
	#[inline] fn as_raw_BaseOCR(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::BaseOCR for PtrOfOCRBeamSearchDecoder {
	#[inline] fn as_raw_mut_BaseOCR(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


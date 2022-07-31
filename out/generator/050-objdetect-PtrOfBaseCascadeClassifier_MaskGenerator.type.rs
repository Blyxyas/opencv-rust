pub type PtrOfBaseCascadeClassifier_MaskGenerator = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator>;

ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier_MaskGenerator,
	cv_PtrOfBaseCascadeClassifier_MaskGenerator_delete, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_MaskGenerator_get_inner_ptr_mut
}

impl PtrOfBaseCascadeClassifier_MaskGenerator {
	#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::BaseCascadeClassifier_MaskGeneratorConst for PtrOfBaseCascadeClassifier_MaskGenerator {
	#[inline] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfBaseCascadeClassifier_MaskGenerator {
	#[inline] fn as_raw_mut_BaseCascadeClassifier_MaskGenerator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


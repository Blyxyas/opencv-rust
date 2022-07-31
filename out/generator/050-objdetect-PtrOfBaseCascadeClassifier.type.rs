pub type PtrOfBaseCascadeClassifier = core::Ptr<dyn crate::objdetect::BaseCascadeClassifier>;

ptr_extern! { dyn crate::objdetect::BaseCascadeClassifier,
	cv_PtrOfBaseCascadeClassifier_delete, cv_PtrOfBaseCascadeClassifier_get_inner_ptr, cv_PtrOfBaseCascadeClassifier_get_inner_ptr_mut
}

impl PtrOfBaseCascadeClassifier {
	#[inline] pub fn as_raw_PtrOfBaseCascadeClassifier(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBaseCascadeClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::objdetect::BaseCascadeClassifierConst for PtrOfBaseCascadeClassifier {
	#[inline] fn as_raw_BaseCascadeClassifier(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::objdetect::BaseCascadeClassifier for PtrOfBaseCascadeClassifier {
	#[inline] fn as_raw_mut_BaseCascadeClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBaseCascadeClassifier {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBaseCascadeClassifier {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


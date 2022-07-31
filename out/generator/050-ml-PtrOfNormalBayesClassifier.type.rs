pub type PtrOfNormalBayesClassifier = core::Ptr<dyn crate::ml::NormalBayesClassifier>;

ptr_extern! { dyn crate::ml::NormalBayesClassifier,
	cv_PtrOfNormalBayesClassifier_delete, cv_PtrOfNormalBayesClassifier_get_inner_ptr, cv_PtrOfNormalBayesClassifier_get_inner_ptr_mut
}

impl PtrOfNormalBayesClassifier {
	#[inline] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfNormalBayesClassifier(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::NormalBayesClassifierConst for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_NormalBayesClassifier(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_mut_NormalBayesClassifier(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


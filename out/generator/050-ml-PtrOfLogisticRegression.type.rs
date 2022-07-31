pub type PtrOfLogisticRegression = core::Ptr<dyn crate::ml::LogisticRegression>;

ptr_extern! { dyn crate::ml::LogisticRegression,
	cv_PtrOfLogisticRegression_delete, cv_PtrOfLogisticRegression_get_inner_ptr, cv_PtrOfLogisticRegression_get_inner_ptr_mut
}

impl PtrOfLogisticRegression {
	#[inline] pub fn as_raw_PtrOfLogisticRegression(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfLogisticRegression(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::ml::LogisticRegressionConst for PtrOfLogisticRegression {
	#[inline] fn as_raw_LogisticRegression(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
	#[inline] fn as_raw_mut_LogisticRegression(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfLogisticRegression {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfLogisticRegression {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::ml::StatModelConst for PtrOfLogisticRegression {
	#[inline] fn as_raw_StatModel(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::ml::StatModel for PtrOfLogisticRegression {
	#[inline] fn as_raw_mut_StatModel(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


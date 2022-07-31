pub type PtrOfThresholdedReluLayer = core::Ptr<crate::dnn::ThresholdedReluLayer>;

ptr_extern! { crate::dnn::ThresholdedReluLayer,
	cv_PtrOfThresholdedReluLayer_delete, cv_PtrOfThresholdedReluLayer_get_inner_ptr, cv_PtrOfThresholdedReluLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ThresholdedReluLayer, cv_PtrOfThresholdedReluLayer_new }

impl PtrOfThresholdedReluLayer {
	#[inline] pub fn as_raw_PtrOfThresholdedReluLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfThresholdedReluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ThresholdedReluLayerTraitConst for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_ThresholdedReluLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ThresholdedReluLayerTrait for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_mut_ThresholdedReluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfThresholdedReluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


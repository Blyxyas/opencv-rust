pub type PtrOfSigmoidLayer = core::Ptr<crate::dnn::SigmoidLayer>;

ptr_extern! { crate::dnn::SigmoidLayer,
	cv_PtrOfSigmoidLayer_delete, cv_PtrOfSigmoidLayer_get_inner_ptr, cv_PtrOfSigmoidLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SigmoidLayer, cv_PtrOfSigmoidLayer_new }

impl PtrOfSigmoidLayer {
	#[inline] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SigmoidLayerTraitConst for PtrOfSigmoidLayer {
	#[inline] fn as_raw_SigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SigmoidLayerTrait for PtrOfSigmoidLayer {
	#[inline] fn as_raw_mut_SigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSigmoidLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSigmoidLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSigmoidLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSigmoidLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSigmoidLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSigmoidLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


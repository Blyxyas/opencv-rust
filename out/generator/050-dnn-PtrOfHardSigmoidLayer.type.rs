pub type PtrOfHardSigmoidLayer = core::Ptr<crate::dnn::HardSigmoidLayer>;

ptr_extern! { crate::dnn::HardSigmoidLayer,
	cv_PtrOfHardSigmoidLayer_delete, cv_PtrOfHardSigmoidLayer_get_inner_ptr, cv_PtrOfHardSigmoidLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::HardSigmoidLayer, cv_PtrOfHardSigmoidLayer_new }

impl PtrOfHardSigmoidLayer {
	#[inline] pub fn as_raw_PtrOfHardSigmoidLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHardSigmoidLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::HardSigmoidLayerTraitConst for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_HardSigmoidLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::HardSigmoidLayerTrait for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_mut_HardSigmoidLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfHardSigmoidLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


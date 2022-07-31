pub type PtrOfPowerLayer = core::Ptr<crate::dnn::PowerLayer>;

ptr_extern! { crate::dnn::PowerLayer,
	cv_PtrOfPowerLayer_delete, cv_PtrOfPowerLayer_get_inner_ptr, cv_PtrOfPowerLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::PowerLayer, cv_PtrOfPowerLayer_new }

impl PtrOfPowerLayer {
	#[inline] pub fn as_raw_PtrOfPowerLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfPowerLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::PowerLayerTraitConst for PtrOfPowerLayer {
	#[inline] fn as_raw_PowerLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::PowerLayerTrait for PtrOfPowerLayer {
	#[inline] fn as_raw_mut_PowerLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfPowerLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfPowerLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfPowerLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfPowerLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfPowerLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfPowerLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


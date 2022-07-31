pub type PtrOfCosLayer = core::Ptr<crate::dnn::CosLayer>;

ptr_extern! { crate::dnn::CosLayer,
	cv_PtrOfCosLayer_delete, cv_PtrOfCosLayer_get_inner_ptr, cv_PtrOfCosLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CosLayer, cv_PtrOfCosLayer_new }

impl PtrOfCosLayer {
	#[inline] pub fn as_raw_PtrOfCosLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CosLayerTraitConst for PtrOfCosLayer {
	#[inline] fn as_raw_CosLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CosLayerTrait for PtrOfCosLayer {
	#[inline] fn as_raw_mut_CosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCosLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCosLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfCosLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfCosLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCosLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCosLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfAcosLayer = core::Ptr<crate::dnn::AcosLayer>;

ptr_extern! { crate::dnn::AcosLayer,
	cv_PtrOfAcosLayer_delete, cv_PtrOfAcosLayer_get_inner_ptr, cv_PtrOfAcosLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AcosLayer, cv_PtrOfAcosLayer_new }

impl PtrOfAcosLayer {
	#[inline] pub fn as_raw_PtrOfAcosLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAcosLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AcosLayerTraitConst for PtrOfAcosLayer {
	#[inline] fn as_raw_AcosLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AcosLayerTrait for PtrOfAcosLayer {
	#[inline] fn as_raw_mut_AcosLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAcosLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAcosLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAcosLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAcosLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAcosLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAcosLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


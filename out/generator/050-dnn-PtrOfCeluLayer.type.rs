pub type PtrOfCeluLayer = core::Ptr<crate::dnn::CeluLayer>;

ptr_extern! { crate::dnn::CeluLayer,
	cv_PtrOfCeluLayer_delete, cv_PtrOfCeluLayer_get_inner_ptr, cv_PtrOfCeluLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CeluLayer, cv_PtrOfCeluLayer_new }

impl PtrOfCeluLayer {
	#[inline] pub fn as_raw_PtrOfCeluLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CeluLayerTraitConst for PtrOfCeluLayer {
	#[inline] fn as_raw_CeluLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CeluLayerTrait for PtrOfCeluLayer {
	#[inline] fn as_raw_mut_CeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCeluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCeluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfCeluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfCeluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCeluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCeluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


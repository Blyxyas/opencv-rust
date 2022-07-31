pub type PtrOfAsinhLayer = core::Ptr<crate::dnn::AsinhLayer>;

ptr_extern! { crate::dnn::AsinhLayer,
	cv_PtrOfAsinhLayer_delete, cv_PtrOfAsinhLayer_get_inner_ptr, cv_PtrOfAsinhLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AsinhLayer, cv_PtrOfAsinhLayer_new }

impl PtrOfAsinhLayer {
	#[inline] pub fn as_raw_PtrOfAsinhLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAsinhLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AsinhLayerTraitConst for PtrOfAsinhLayer {
	#[inline] fn as_raw_AsinhLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AsinhLayerTrait for PtrOfAsinhLayer {
	#[inline] fn as_raw_mut_AsinhLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAsinhLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAsinhLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAsinhLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAsinhLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAsinhLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAsinhLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfQuantizeLayer = core::Ptr<crate::dnn::QuantizeLayer>;

ptr_extern! { crate::dnn::QuantizeLayer,
	cv_PtrOfQuantizeLayer_delete, cv_PtrOfQuantizeLayer_get_inner_ptr, cv_PtrOfQuantizeLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::QuantizeLayer, cv_PtrOfQuantizeLayer_new }

impl PtrOfQuantizeLayer {
	#[inline] pub fn as_raw_PtrOfQuantizeLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfQuantizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::QuantizeLayerTraitConst for PtrOfQuantizeLayer {
	#[inline] fn as_raw_QuantizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::QuantizeLayerTrait for PtrOfQuantizeLayer {
	#[inline] fn as_raw_mut_QuantizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfQuantizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfQuantizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfQuantizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfQuantizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


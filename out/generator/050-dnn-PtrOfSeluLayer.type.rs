pub type PtrOfSeluLayer = core::Ptr<crate::dnn::SeluLayer>;

ptr_extern! { crate::dnn::SeluLayer,
	cv_PtrOfSeluLayer_delete, cv_PtrOfSeluLayer_get_inner_ptr, cv_PtrOfSeluLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SeluLayer, cv_PtrOfSeluLayer_new }

impl PtrOfSeluLayer {
	#[inline] pub fn as_raw_PtrOfSeluLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSeluLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SeluLayerTraitConst for PtrOfSeluLayer {
	#[inline] fn as_raw_SeluLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SeluLayerTrait for PtrOfSeluLayer {
	#[inline] fn as_raw_mut_SeluLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSeluLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSeluLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSeluLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSeluLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSeluLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSeluLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


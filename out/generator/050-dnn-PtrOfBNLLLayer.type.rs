pub type PtrOfBNLLLayer = core::Ptr<crate::dnn::BNLLLayer>;

ptr_extern! { crate::dnn::BNLLLayer,
	cv_PtrOfBNLLLayer_delete, cv_PtrOfBNLLLayer_get_inner_ptr, cv_PtrOfBNLLLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::BNLLLayer, cv_PtrOfBNLLLayer_new }

impl PtrOfBNLLLayer {
	#[inline] pub fn as_raw_PtrOfBNLLLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBNLLLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::BNLLLayerTraitConst for PtrOfBNLLLayer {
	#[inline] fn as_raw_BNLLLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::BNLLLayerTrait for PtrOfBNLLLayer {
	#[inline] fn as_raw_mut_BNLLLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfBNLLLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfBNLLLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfBNLLLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfBNLLLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfBNLLLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfBNLLLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


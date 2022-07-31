pub type PtrOfReLULayer = core::Ptr<crate::dnn::ReLULayer>;

ptr_extern! { crate::dnn::ReLULayer,
	cv_PtrOfReLULayer_delete, cv_PtrOfReLULayer_get_inner_ptr, cv_PtrOfReLULayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReLULayer, cv_PtrOfReLULayer_new }

impl PtrOfReLULayer {
	#[inline] pub fn as_raw_PtrOfReLULayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReLULayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReLULayerTraitConst for PtrOfReLULayer {
	#[inline] fn as_raw_ReLULayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReLULayerTrait for PtrOfReLULayer {
	#[inline] fn as_raw_mut_ReLULayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReLULayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReLULayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfReLULayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfReLULayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReLULayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReLULayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfReLU6Layer = core::Ptr<crate::dnn::ReLU6Layer>;

ptr_extern! { crate::dnn::ReLU6Layer,
	cv_PtrOfReLU6Layer_delete, cv_PtrOfReLU6Layer_get_inner_ptr, cv_PtrOfReLU6Layer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReLU6Layer, cv_PtrOfReLU6Layer_new }

impl PtrOfReLU6Layer {
	#[inline] pub fn as_raw_PtrOfReLU6Layer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReLU6Layer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReLU6LayerTraitConst for PtrOfReLU6Layer {
	#[inline] fn as_raw_ReLU6Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReLU6LayerTrait for PtrOfReLU6Layer {
	#[inline] fn as_raw_mut_ReLU6Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReLU6Layer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReLU6Layer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfReLU6Layer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfReLU6Layer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReLU6Layer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReLU6Layer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


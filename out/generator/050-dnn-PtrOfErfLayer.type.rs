pub type PtrOfErfLayer = core::Ptr<crate::dnn::ErfLayer>;

ptr_extern! { crate::dnn::ErfLayer,
	cv_PtrOfErfLayer_delete, cv_PtrOfErfLayer_get_inner_ptr, cv_PtrOfErfLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ErfLayer, cv_PtrOfErfLayer_new }

impl PtrOfErfLayer {
	#[inline] pub fn as_raw_PtrOfErfLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfErfLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ErfLayerTraitConst for PtrOfErfLayer {
	#[inline] fn as_raw_ErfLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ErfLayerTrait for PtrOfErfLayer {
	#[inline] fn as_raw_mut_ErfLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfErfLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfErfLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfErfLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfErfLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfErfLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfErfLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


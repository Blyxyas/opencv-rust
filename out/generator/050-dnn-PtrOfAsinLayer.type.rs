pub type PtrOfAsinLayer = core::Ptr<crate::dnn::AsinLayer>;

ptr_extern! { crate::dnn::AsinLayer,
	cv_PtrOfAsinLayer_delete, cv_PtrOfAsinLayer_get_inner_ptr, cv_PtrOfAsinLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AsinLayer, cv_PtrOfAsinLayer_new }

impl PtrOfAsinLayer {
	#[inline] pub fn as_raw_PtrOfAsinLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAsinLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AsinLayerTraitConst for PtrOfAsinLayer {
	#[inline] fn as_raw_AsinLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AsinLayerTrait for PtrOfAsinLayer {
	#[inline] fn as_raw_mut_AsinLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAsinLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAsinLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAsinLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAsinLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAsinLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAsinLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


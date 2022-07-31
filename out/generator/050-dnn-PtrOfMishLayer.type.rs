pub type PtrOfMishLayer = core::Ptr<crate::dnn::MishLayer>;

ptr_extern! { crate::dnn::MishLayer,
	cv_PtrOfMishLayer_delete, cv_PtrOfMishLayer_get_inner_ptr, cv_PtrOfMishLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::MishLayer, cv_PtrOfMishLayer_new }

impl PtrOfMishLayer {
	#[inline] pub fn as_raw_PtrOfMishLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMishLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::MishLayerTraitConst for PtrOfMishLayer {
	#[inline] fn as_raw_MishLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::MishLayerTrait for PtrOfMishLayer {
	#[inline] fn as_raw_mut_MishLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfMishLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfMishLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfMishLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfMishLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfMishLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfMishLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


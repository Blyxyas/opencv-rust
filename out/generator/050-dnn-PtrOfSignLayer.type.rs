pub type PtrOfSignLayer = core::Ptr<crate::dnn::SignLayer>;

ptr_extern! { crate::dnn::SignLayer,
	cv_PtrOfSignLayer_delete, cv_PtrOfSignLayer_get_inner_ptr, cv_PtrOfSignLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SignLayer, cv_PtrOfSignLayer_new }

impl PtrOfSignLayer {
	#[inline] pub fn as_raw_PtrOfSignLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSignLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SignLayerTraitConst for PtrOfSignLayer {
	#[inline] fn as_raw_SignLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SignLayerTrait for PtrOfSignLayer {
	#[inline] fn as_raw_mut_SignLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSignLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSignLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSignLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSignLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSignLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSignLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


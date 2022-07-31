pub type PtrOfExpLayer = core::Ptr<crate::dnn::ExpLayer>;

ptr_extern! { crate::dnn::ExpLayer,
	cv_PtrOfExpLayer_delete, cv_PtrOfExpLayer_get_inner_ptr, cv_PtrOfExpLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ExpLayer, cv_PtrOfExpLayer_new }

impl PtrOfExpLayer {
	#[inline] pub fn as_raw_PtrOfExpLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfExpLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ExpLayerTraitConst for PtrOfExpLayer {
	#[inline] fn as_raw_ExpLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ExpLayerTrait for PtrOfExpLayer {
	#[inline] fn as_raw_mut_ExpLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfExpLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfExpLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfExpLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfExpLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfExpLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfExpLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfAcoshLayer = core::Ptr<crate::dnn::AcoshLayer>;

ptr_extern! { crate::dnn::AcoshLayer,
	cv_PtrOfAcoshLayer_delete, cv_PtrOfAcoshLayer_get_inner_ptr, cv_PtrOfAcoshLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::AcoshLayer, cv_PtrOfAcoshLayer_new }

impl PtrOfAcoshLayer {
	#[inline] pub fn as_raw_PtrOfAcoshLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAcoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::AcoshLayerTraitConst for PtrOfAcoshLayer {
	#[inline] fn as_raw_AcoshLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::AcoshLayerTrait for PtrOfAcoshLayer {
	#[inline] fn as_raw_mut_AcoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfAcoshLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfAcoshLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfAcoshLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfAcoshLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfAcoshLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfAcoshLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


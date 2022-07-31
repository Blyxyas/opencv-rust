pub type PtrOfCoshLayer = core::Ptr<crate::dnn::CoshLayer>;

ptr_extern! { crate::dnn::CoshLayer,
	cv_PtrOfCoshLayer_delete, cv_PtrOfCoshLayer_get_inner_ptr, cv_PtrOfCoshLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::CoshLayer, cv_PtrOfCoshLayer_new }

impl PtrOfCoshLayer {
	#[inline] pub fn as_raw_PtrOfCoshLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfCoshLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::CoshLayerTraitConst for PtrOfCoshLayer {
	#[inline] fn as_raw_CoshLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::CoshLayerTrait for PtrOfCoshLayer {
	#[inline] fn as_raw_mut_CoshLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfCoshLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfCoshLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfCoshLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfCoshLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfCoshLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfCoshLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


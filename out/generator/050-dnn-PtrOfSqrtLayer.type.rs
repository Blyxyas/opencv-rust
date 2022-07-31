pub type PtrOfSqrtLayer = core::Ptr<crate::dnn::SqrtLayer>;

ptr_extern! { crate::dnn::SqrtLayer,
	cv_PtrOfSqrtLayer_delete, cv_PtrOfSqrtLayer_get_inner_ptr, cv_PtrOfSqrtLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::SqrtLayer, cv_PtrOfSqrtLayer_new }

impl PtrOfSqrtLayer {
	#[inline] pub fn as_raw_PtrOfSqrtLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfSqrtLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::SqrtLayerTraitConst for PtrOfSqrtLayer {
	#[inline] fn as_raw_SqrtLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::SqrtLayerTrait for PtrOfSqrtLayer {
	#[inline] fn as_raw_mut_SqrtLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfSqrtLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfSqrtLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfSqrtLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfSqrtLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfSqrtLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfSqrtLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


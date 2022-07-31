pub type PtrOfReciprocalLayer = core::Ptr<crate::dnn::ReciprocalLayer>;

ptr_extern! { crate::dnn::ReciprocalLayer,
	cv_PtrOfReciprocalLayer_delete, cv_PtrOfReciprocalLayer_get_inner_ptr, cv_PtrOfReciprocalLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReciprocalLayer, cv_PtrOfReciprocalLayer_new }

impl PtrOfReciprocalLayer {
	#[inline] pub fn as_raw_PtrOfReciprocalLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReciprocalLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReciprocalLayerTraitConst for PtrOfReciprocalLayer {
	#[inline] fn as_raw_ReciprocalLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReciprocalLayerTrait for PtrOfReciprocalLayer {
	#[inline] fn as_raw_mut_ReciprocalLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReciprocalLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReciprocalLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::ActivationLayerTraitConst for PtrOfReciprocalLayer {
	#[inline] fn as_raw_ActivationLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ActivationLayerTrait for PtrOfReciprocalLayer {
	#[inline] fn as_raw_mut_ActivationLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReciprocalLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReciprocalLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


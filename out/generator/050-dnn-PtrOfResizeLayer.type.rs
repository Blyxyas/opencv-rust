pub type PtrOfResizeLayer = core::Ptr<crate::dnn::ResizeLayer>;

ptr_extern! { crate::dnn::ResizeLayer,
	cv_PtrOfResizeLayer_delete, cv_PtrOfResizeLayer_get_inner_ptr, cv_PtrOfResizeLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ResizeLayer, cv_PtrOfResizeLayer_new }

impl PtrOfResizeLayer {
	#[inline] pub fn as_raw_PtrOfResizeLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfResizeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ResizeLayerTraitConst for PtrOfResizeLayer {
	#[inline] fn as_raw_ResizeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ResizeLayerTrait for PtrOfResizeLayer {
	#[inline] fn as_raw_mut_ResizeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfResizeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfResizeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfResizeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfResizeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


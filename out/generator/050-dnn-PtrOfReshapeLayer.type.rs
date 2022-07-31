pub type PtrOfReshapeLayer = core::Ptr<crate::dnn::ReshapeLayer>;

ptr_extern! { crate::dnn::ReshapeLayer,
	cv_PtrOfReshapeLayer_delete, cv_PtrOfReshapeLayer_get_inner_ptr, cv_PtrOfReshapeLayer_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::dnn::ReshapeLayer, cv_PtrOfReshapeLayer_new }

impl PtrOfReshapeLayer {
	#[inline] pub fn as_raw_PtrOfReshapeLayer(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfReshapeLayer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::dnn::ReshapeLayerTraitConst for PtrOfReshapeLayer {
	#[inline] fn as_raw_ReshapeLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ReshapeLayerTrait for PtrOfReshapeLayer {
	#[inline] fn as_raw_mut_ReshapeLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfReshapeLayer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfReshapeLayer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::dnn::LayerTraitConst for PtrOfReshapeLayer {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for PtrOfReshapeLayer {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfObjectnessBING = core::Ptr<crate::saliency::ObjectnessBING>;

ptr_extern! { crate::saliency::ObjectnessBING,
	cv_PtrOfObjectnessBING_delete, cv_PtrOfObjectnessBING_get_inner_ptr, cv_PtrOfObjectnessBING_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::saliency::ObjectnessBING, cv_PtrOfObjectnessBING_new }

impl PtrOfObjectnessBING {
	#[inline] pub fn as_raw_PtrOfObjectnessBING(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessBINGTraitConst for PtrOfObjectnessBING {
	#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::ObjectnessBINGTrait for PtrOfObjectnessBING {
	#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for PtrOfObjectnessBING {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for PtrOfObjectnessBING {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::ObjectnessConst for PtrOfObjectnessBING {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::Objectness for PtrOfObjectnessBING {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::saliency::SaliencyConst for PtrOfObjectnessBING {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::Saliency for PtrOfObjectnessBING {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


pub type PtrOfERFilter_Callback = core::Ptr<dyn crate::text::ERFilter_Callback>;

ptr_extern! { dyn crate::text::ERFilter_Callback,
	cv_PtrOfERFilter_Callback_delete, cv_PtrOfERFilter_Callback_get_inner_ptr, cv_PtrOfERFilter_Callback_get_inner_ptr_mut
}

impl PtrOfERFilter_Callback {
	#[inline] pub fn as_raw_PtrOfERFilter_Callback(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfERFilter_Callback(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::text::ERFilter_CallbackConst for PtrOfERFilter_Callback {
	#[inline] fn as_raw_ERFilter_Callback(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::text::ERFilter_Callback for PtrOfERFilter_Callback {
	#[inline] fn as_raw_mut_ERFilter_Callback(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


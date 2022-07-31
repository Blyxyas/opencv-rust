pub type PtrOff32 = core::Ptr<f32>;

ptr_extern! { f32,
	cv_PtrOff32_delete, cv_PtrOff32_get_inner_ptr, cv_PtrOff32_get_inner_ptr_mut
}

ptr_extern_ctor! { f32, cv_PtrOff32_new }

impl PtrOff32 {
	#[inline] pub fn as_raw_PtrOff32(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
}


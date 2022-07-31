pub type PtrOfMotionInpainter = core::Ptr<crate::videostab::MotionInpainter>;

ptr_extern! { crate::videostab::MotionInpainter,
	cv_PtrOfMotionInpainter_delete, cv_PtrOfMotionInpainter_get_inner_ptr, cv_PtrOfMotionInpainter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::MotionInpainter, cv_PtrOfMotionInpainter_new }

impl PtrOfMotionInpainter {
	#[inline] pub fn as_raw_PtrOfMotionInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMotionInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::MotionInpainterTraitConst for PtrOfMotionInpainter {
	#[inline] fn as_raw_MotionInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::MotionInpainterTrait for PtrOfMotionInpainter {
	#[inline] fn as_raw_mut_MotionInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfMotionInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfMotionInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMotionInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfMotionInpainter_to_PtrOfInpainterBase,
}


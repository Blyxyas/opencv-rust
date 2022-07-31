pub type PtrOfConsistentMosaicInpainter = core::Ptr<crate::videostab::ConsistentMosaicInpainter>;

ptr_extern! { crate::videostab::ConsistentMosaicInpainter,
	cv_PtrOfConsistentMosaicInpainter_delete, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr, cv_PtrOfConsistentMosaicInpainter_get_inner_ptr_mut
}

ptr_extern_ctor! { crate::videostab::ConsistentMosaicInpainter, cv_PtrOfConsistentMosaicInpainter_new }

impl PtrOfConsistentMosaicInpainter {
	#[inline] pub fn as_raw_PtrOfConsistentMosaicInpainter(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConsistentMosaicInpainter(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::videostab::ConsistentMosaicInpainterTraitConst for PtrOfConsistentMosaicInpainter {
	#[inline] fn as_raw_ConsistentMosaicInpainter(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::ConsistentMosaicInpainterTrait for PtrOfConsistentMosaicInpainter {
	#[inline] fn as_raw_mut_ConsistentMosaicInpainter(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::videostab::InpainterBaseConst for PtrOfConsistentMosaicInpainter {
	#[inline] fn as_raw_InpainterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::videostab::InpainterBase for PtrOfConsistentMosaicInpainter {
	#[inline] fn as_raw_mut_InpainterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfConsistentMosaicInpainter, core::Ptr<dyn crate::videostab::InpainterBase>,
	cv_PtrOfConsistentMosaicInpainter_to_PtrOfInpainterBase,
}


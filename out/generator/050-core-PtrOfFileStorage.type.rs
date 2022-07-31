pub type PtrOfFileStorage = core::Ptr<core::FileStorage>;

ptr_extern! { core::FileStorage,
	cv_PtrOfFileStorage_delete, cv_PtrOfFileStorage_get_inner_ptr, cv_PtrOfFileStorage_get_inner_ptr_mut
}

ptr_extern_ctor! { core::FileStorage, cv_PtrOfFileStorage_new }

impl PtrOfFileStorage {
	#[inline] pub fn as_raw_PtrOfFileStorage(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfFileStorage(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::FileStorageTraitConst for PtrOfFileStorage {
	#[inline] fn as_raw_FileStorage(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::FileStorageTrait for PtrOfFileStorage {
	#[inline] fn as_raw_mut_FileStorage(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


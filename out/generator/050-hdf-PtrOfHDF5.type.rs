pub type PtrOfHDF5 = core::Ptr<dyn crate::hdf::HDF5>;

ptr_extern! { dyn crate::hdf::HDF5,
	cv_PtrOfHDF5_delete, cv_PtrOfHDF5_get_inner_ptr, cv_PtrOfHDF5_get_inner_ptr_mut
}

impl PtrOfHDF5 {
	#[inline] pub fn as_raw_PtrOfHDF5(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfHDF5(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::hdf::HDF5Const for PtrOfHDF5 {
	#[inline] fn as_raw_HDF5(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::hdf::HDF5 for PtrOfHDF5 {
	#[inline] fn as_raw_mut_HDF5(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


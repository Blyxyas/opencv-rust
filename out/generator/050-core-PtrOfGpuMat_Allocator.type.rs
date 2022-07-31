pub type PtrOfGpuMat_Allocator = core::Ptr<dyn core::GpuMat_Allocator>;

ptr_extern! { dyn core::GpuMat_Allocator,
	cv_PtrOfGpuMat_Allocator_delete, cv_PtrOfGpuMat_Allocator_get_inner_ptr, cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut
}

impl PtrOfGpuMat_Allocator {
	#[inline] pub fn as_raw_PtrOfGpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfGpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl core::GpuMat_AllocatorConst for PtrOfGpuMat_Allocator {
	#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::GpuMat_Allocator for PtrOfGpuMat_Allocator {
	#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}


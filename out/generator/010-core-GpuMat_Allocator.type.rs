impl core::GpuMat_AllocatorConst for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
	#[inline] fn as_raw_GpuMat_Allocator(&self) -> *const c_void { self.as_raw() }
}

impl core::GpuMat_Allocator for types::AbstractRefMut<'static, dyn core::GpuMat_Allocator> {
	#[inline] fn as_raw_mut_GpuMat_Allocator(&mut self) -> *mut c_void { self.as_raw_mut() }
}


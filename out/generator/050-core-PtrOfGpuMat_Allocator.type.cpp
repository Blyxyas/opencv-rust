extern "C" {
	void cv_PtrOfGpuMat_Allocator_delete(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
		delete instance;
	}

	const cv::cuda::GpuMat::Allocator* cv_PtrOfGpuMat_Allocator_get_inner_ptr(const cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
		return instance->get();
	}

	cv::cuda::GpuMat::Allocator* cv_PtrOfGpuMat_Allocator_get_inner_ptr_mut(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
		return instance->get();
	}
}


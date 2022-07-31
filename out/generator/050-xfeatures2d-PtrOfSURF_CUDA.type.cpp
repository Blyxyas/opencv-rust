extern "C" {
	cv::Ptr<cv::cuda::SURF_CUDA>* cv_PtrOfSURF_CUDA_new(cv::cuda::SURF_CUDA* val) {
		return new cv::Ptr<cv::cuda::SURF_CUDA>(val);
	}
	
	void cv_PtrOfSURF_CUDA_delete(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
		delete instance;
	}

	const cv::cuda::SURF_CUDA* cv_PtrOfSURF_CUDA_get_inner_ptr(const cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
		return instance->get();
	}

	cv::cuda::SURF_CUDA* cv_PtrOfSURF_CUDA_get_inner_ptr_mut(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
		return instance->get();
	}
}


extern "C" {
	void cv_PtrOfBackendWrapper_delete(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		delete instance;
	}

	const cv::dnn::BackendWrapper* cv_PtrOfBackendWrapper_get_inner_ptr(const cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		return instance->get();
	}

	cv::dnn::BackendWrapper* cv_PtrOfBackendWrapper_get_inner_ptr_mut(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
		return instance->get();
	}
}


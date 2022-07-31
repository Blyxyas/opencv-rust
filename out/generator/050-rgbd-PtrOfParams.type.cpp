extern "C" {
	cv::Ptr<cv::large_kinfu::Params>* cv_PtrOfParams_new(cv::large_kinfu::Params* val) {
		return new cv::Ptr<cv::large_kinfu::Params>(val);
	}
	
	void cv_PtrOfParams_delete(cv::Ptr<cv::large_kinfu::Params>* instance) {
		delete instance;
	}

	const cv::large_kinfu::Params* cv_PtrOfParams_get_inner_ptr(const cv::Ptr<cv::large_kinfu::Params>* instance) {
		return instance->get();
	}

	cv::large_kinfu::Params* cv_PtrOfParams_get_inner_ptr_mut(cv::Ptr<cv::large_kinfu::Params>* instance) {
		return instance->get();
	}
}


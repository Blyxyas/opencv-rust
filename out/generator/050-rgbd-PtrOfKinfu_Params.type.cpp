extern "C" {
	cv::Ptr<cv::kinfu::Params>* cv_PtrOfKinfu_Params_new(cv::kinfu::Params* val) {
		return new cv::Ptr<cv::kinfu::Params>(val);
	}
	
	void cv_PtrOfKinfu_Params_delete(cv::Ptr<cv::kinfu::Params>* instance) {
		delete instance;
	}

	const cv::kinfu::Params* cv_PtrOfKinfu_Params_get_inner_ptr(const cv::Ptr<cv::kinfu::Params>* instance) {
		return instance->get();
	}

	cv::kinfu::Params* cv_PtrOfKinfu_Params_get_inner_ptr_mut(cv::Ptr<cv::kinfu::Params>* instance) {
		return instance->get();
	}
}


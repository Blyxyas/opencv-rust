extern "C" {
	cv::Ptr<cv::colored_kinfu::Params>* cv_PtrOfColoredKinfu_Params_new(cv::colored_kinfu::Params* val) {
		return new cv::Ptr<cv::colored_kinfu::Params>(val);
	}
	
	void cv_PtrOfColoredKinfu_Params_delete(cv::Ptr<cv::colored_kinfu::Params>* instance) {
		delete instance;
	}

	const cv::colored_kinfu::Params* cv_PtrOfColoredKinfu_Params_get_inner_ptr(const cv::Ptr<cv::colored_kinfu::Params>* instance) {
		return instance->get();
	}

	cv::colored_kinfu::Params* cv_PtrOfColoredKinfu_Params_get_inner_ptr_mut(cv::Ptr<cv::colored_kinfu::Params>* instance) {
		return instance->get();
	}
}


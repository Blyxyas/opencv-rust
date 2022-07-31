extern "C" {
	cv::Ptr<cv::dnn::ActivationLayer>* cv_PtrOfActivationLayer_new(cv::dnn::ActivationLayer* val) {
		return new cv::Ptr<cv::dnn::ActivationLayer>(val);
	}
	
	void cv_PtrOfActivationLayer_delete(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ActivationLayer* cv_PtrOfActivationLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ActivationLayer* cv_PtrOfActivationLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ActivationLayer>* instance) {
		return instance->get();
	}
}


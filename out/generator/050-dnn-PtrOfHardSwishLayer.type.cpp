extern "C" {
	cv::Ptr<cv::dnn::HardSwishLayer>* cv_PtrOfHardSwishLayer_new(cv::dnn::HardSwishLayer* val) {
		return new cv::Ptr<cv::dnn::HardSwishLayer>(val);
	}
	
	void cv_PtrOfHardSwishLayer_delete(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
		delete instance;
	}

	const cv::dnn::HardSwishLayer* cv_PtrOfHardSwishLayer_get_inner_ptr(const cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
		return instance->get();
	}

	cv::dnn::HardSwishLayer* cv_PtrOfHardSwishLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::HardSwishLayer>* instance) {
		return instance->get();
	}
}


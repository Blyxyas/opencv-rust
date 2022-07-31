extern "C" {
	cv::Ptr<cv::dnn::BNLLLayer>* cv_PtrOfBNLLLayer_new(cv::dnn::BNLLLayer* val) {
		return new cv::Ptr<cv::dnn::BNLLLayer>(val);
	}
	
	void cv_PtrOfBNLLLayer_delete(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		delete instance;
	}

	const cv::dnn::BNLLLayer* cv_PtrOfBNLLLayer_get_inner_ptr(const cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		return instance->get();
	}

	cv::dnn::BNLLLayer* cv_PtrOfBNLLLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::BNLLLayer>* instance) {
		return instance->get();
	}
}


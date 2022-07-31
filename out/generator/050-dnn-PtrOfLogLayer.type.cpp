extern "C" {
	cv::Ptr<cv::dnn::LogLayer>* cv_PtrOfLogLayer_new(cv::dnn::LogLayer* val) {
		return new cv::Ptr<cv::dnn::LogLayer>(val);
	}
	
	void cv_PtrOfLogLayer_delete(cv::Ptr<cv::dnn::LogLayer>* instance) {
		delete instance;
	}

	const cv::dnn::LogLayer* cv_PtrOfLogLayer_get_inner_ptr(const cv::Ptr<cv::dnn::LogLayer>* instance) {
		return instance->get();
	}

	cv::dnn::LogLayer* cv_PtrOfLogLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::LogLayer>* instance) {
		return instance->get();
	}
}


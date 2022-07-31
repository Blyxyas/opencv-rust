extern "C" {
	cv::Ptr<cv::dnn::ArgLayer>* cv_PtrOfArgLayer_new(cv::dnn::ArgLayer* val) {
		return new cv::Ptr<cv::dnn::ArgLayer>(val);
	}
	
	void cv_PtrOfArgLayer_delete(cv::Ptr<cv::dnn::ArgLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ArgLayer* cv_PtrOfArgLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ArgLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ArgLayer* cv_PtrOfArgLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ArgLayer>* instance) {
		return instance->get();
	}
}


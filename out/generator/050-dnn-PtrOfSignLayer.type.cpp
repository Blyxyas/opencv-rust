extern "C" {
	cv::Ptr<cv::dnn::SignLayer>* cv_PtrOfSignLayer_new(cv::dnn::SignLayer* val) {
		return new cv::Ptr<cv::dnn::SignLayer>(val);
	}
	
	void cv_PtrOfSignLayer_delete(cv::Ptr<cv::dnn::SignLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SignLayer* cv_PtrOfSignLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SignLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SignLayer* cv_PtrOfSignLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SignLayer>* instance) {
		return instance->get();
	}
}


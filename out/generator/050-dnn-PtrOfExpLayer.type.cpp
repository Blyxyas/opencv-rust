extern "C" {
	cv::Ptr<cv::dnn::ExpLayer>* cv_PtrOfExpLayer_new(cv::dnn::ExpLayer* val) {
		return new cv::Ptr<cv::dnn::ExpLayer>(val);
	}
	
	void cv_PtrOfExpLayer_delete(cv::Ptr<cv::dnn::ExpLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ExpLayer* cv_PtrOfExpLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ExpLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ExpLayer* cv_PtrOfExpLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ExpLayer>* instance) {
		return instance->get();
	}
}


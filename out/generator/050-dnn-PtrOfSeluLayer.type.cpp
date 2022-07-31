extern "C" {
	cv::Ptr<cv::dnn::SeluLayer>* cv_PtrOfSeluLayer_new(cv::dnn::SeluLayer* val) {
		return new cv::Ptr<cv::dnn::SeluLayer>(val);
	}
	
	void cv_PtrOfSeluLayer_delete(cv::Ptr<cv::dnn::SeluLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SeluLayer* cv_PtrOfSeluLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SeluLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SeluLayer* cv_PtrOfSeluLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SeluLayer>* instance) {
		return instance->get();
	}
}


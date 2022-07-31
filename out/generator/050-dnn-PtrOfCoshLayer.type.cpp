extern "C" {
	cv::Ptr<cv::dnn::CoshLayer>* cv_PtrOfCoshLayer_new(cv::dnn::CoshLayer* val) {
		return new cv::Ptr<cv::dnn::CoshLayer>(val);
	}
	
	void cv_PtrOfCoshLayer_delete(cv::Ptr<cv::dnn::CoshLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CoshLayer* cv_PtrOfCoshLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CoshLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CoshLayer* cv_PtrOfCoshLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CoshLayer>* instance) {
		return instance->get();
	}
}


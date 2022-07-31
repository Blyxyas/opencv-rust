extern "C" {
	cv::Ptr<cv::dnn::ELULayer>* cv_PtrOfELULayer_new(cv::dnn::ELULayer* val) {
		return new cv::Ptr<cv::dnn::ELULayer>(val);
	}
	
	void cv_PtrOfELULayer_delete(cv::Ptr<cv::dnn::ELULayer>* instance) {
		delete instance;
	}

	const cv::dnn::ELULayer* cv_PtrOfELULayer_get_inner_ptr(const cv::Ptr<cv::dnn::ELULayer>* instance) {
		return instance->get();
	}

	cv::dnn::ELULayer* cv_PtrOfELULayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ELULayer>* instance) {
		return instance->get();
	}
}


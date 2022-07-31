extern "C" {
	cv::Ptr<cv::dnn::PaddingLayer>* cv_PtrOfPaddingLayer_new(cv::dnn::PaddingLayer* val) {
		return new cv::Ptr<cv::dnn::PaddingLayer>(val);
	}
	
	void cv_PtrOfPaddingLayer_delete(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PaddingLayer* cv_PtrOfPaddingLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PaddingLayer* cv_PtrOfPaddingLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PaddingLayer>* instance) {
		return instance->get();
	}
}


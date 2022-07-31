extern "C" {
	cv::Ptr<cv::dnn::SqrtLayer>* cv_PtrOfSqrtLayer_new(cv::dnn::SqrtLayer* val) {
		return new cv::Ptr<cv::dnn::SqrtLayer>(val);
	}
	
	void cv_PtrOfSqrtLayer_delete(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SqrtLayer* cv_PtrOfSqrtLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SqrtLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SqrtLayer* cv_PtrOfSqrtLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SqrtLayer>* instance) {
		return instance->get();
	}
}


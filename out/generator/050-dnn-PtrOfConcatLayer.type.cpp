extern "C" {
	cv::Ptr<cv::dnn::ConcatLayer>* cv_PtrOfConcatLayer_new(cv::dnn::ConcatLayer* val) {
		return new cv::Ptr<cv::dnn::ConcatLayer>(val);
	}
	
	void cv_PtrOfConcatLayer_delete(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ConcatLayer* cv_PtrOfConcatLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ConcatLayer* cv_PtrOfConcatLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ConcatLayer>* instance) {
		return instance->get();
	}
}


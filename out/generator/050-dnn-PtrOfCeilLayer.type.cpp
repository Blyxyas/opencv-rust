extern "C" {
	cv::Ptr<cv::dnn::CeilLayer>* cv_PtrOfCeilLayer_new(cv::dnn::CeilLayer* val) {
		return new cv::Ptr<cv::dnn::CeilLayer>(val);
	}
	
	void cv_PtrOfCeilLayer_delete(cv::Ptr<cv::dnn::CeilLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CeilLayer* cv_PtrOfCeilLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CeilLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CeilLayer* cv_PtrOfCeilLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CeilLayer>* instance) {
		return instance->get();
	}
}


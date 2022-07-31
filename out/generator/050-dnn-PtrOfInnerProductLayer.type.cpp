extern "C" {
	cv::Ptr<cv::dnn::InnerProductLayer>* cv_PtrOfInnerProductLayer_new(cv::dnn::InnerProductLayer* val) {
		return new cv::Ptr<cv::dnn::InnerProductLayer>(val);
	}
	
	void cv_PtrOfInnerProductLayer_delete(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		delete instance;
	}

	const cv::dnn::InnerProductLayer* cv_PtrOfInnerProductLayer_get_inner_ptr(const cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		return instance->get();
	}

	cv::dnn::InnerProductLayer* cv_PtrOfInnerProductLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::InnerProductLayer>* instance) {
		return instance->get();
	}
}


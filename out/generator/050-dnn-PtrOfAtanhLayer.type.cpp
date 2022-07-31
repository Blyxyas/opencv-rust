extern "C" {
	cv::Ptr<cv::dnn::AtanhLayer>* cv_PtrOfAtanhLayer_new(cv::dnn::AtanhLayer* val) {
		return new cv::Ptr<cv::dnn::AtanhLayer>(val);
	}
	
	void cv_PtrOfAtanhLayer_delete(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AtanhLayer* cv_PtrOfAtanhLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AtanhLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AtanhLayer* cv_PtrOfAtanhLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AtanhLayer>* instance) {
		return instance->get();
	}
}


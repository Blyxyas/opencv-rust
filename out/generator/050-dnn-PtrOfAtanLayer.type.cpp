extern "C" {
	cv::Ptr<cv::dnn::AtanLayer>* cv_PtrOfAtanLayer_new(cv::dnn::AtanLayer* val) {
		return new cv::Ptr<cv::dnn::AtanLayer>(val);
	}
	
	void cv_PtrOfAtanLayer_delete(cv::Ptr<cv::dnn::AtanLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AtanLayer* cv_PtrOfAtanLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AtanLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AtanLayer* cv_PtrOfAtanLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AtanLayer>* instance) {
		return instance->get();
	}
}


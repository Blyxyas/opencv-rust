extern "C" {
	cv::Ptr<cv::dnn::SoftsignLayer>* cv_PtrOfSoftsignLayer_new(cv::dnn::SoftsignLayer* val) {
		return new cv::Ptr<cv::dnn::SoftsignLayer>(val);
	}
	
	void cv_PtrOfSoftsignLayer_delete(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SoftsignLayer* cv_PtrOfSoftsignLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SoftsignLayer* cv_PtrOfSoftsignLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SoftsignLayer>* instance) {
		return instance->get();
	}
}


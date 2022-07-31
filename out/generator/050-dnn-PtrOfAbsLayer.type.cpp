extern "C" {
	cv::Ptr<cv::dnn::AbsLayer>* cv_PtrOfAbsLayer_new(cv::dnn::AbsLayer* val) {
		return new cv::Ptr<cv::dnn::AbsLayer>(val);
	}
	
	void cv_PtrOfAbsLayer_delete(cv::Ptr<cv::dnn::AbsLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AbsLayer* cv_PtrOfAbsLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AbsLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AbsLayer* cv_PtrOfAbsLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AbsLayer>* instance) {
		return instance->get();
	}
}


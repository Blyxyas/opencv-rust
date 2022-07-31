extern "C" {
	cv::Ptr<cv::dnn::SinLayer>* cv_PtrOfSinLayer_new(cv::dnn::SinLayer* val) {
		return new cv::Ptr<cv::dnn::SinLayer>(val);
	}
	
	void cv_PtrOfSinLayer_delete(cv::Ptr<cv::dnn::SinLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SinLayer* cv_PtrOfSinLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SinLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SinLayer* cv_PtrOfSinLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SinLayer>* instance) {
		return instance->get();
	}
}


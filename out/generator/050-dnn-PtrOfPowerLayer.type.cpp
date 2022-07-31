extern "C" {
	cv::Ptr<cv::dnn::PowerLayer>* cv_PtrOfPowerLayer_new(cv::dnn::PowerLayer* val) {
		return new cv::Ptr<cv::dnn::PowerLayer>(val);
	}
	
	void cv_PtrOfPowerLayer_delete(cv::Ptr<cv::dnn::PowerLayer>* instance) {
		delete instance;
	}

	const cv::dnn::PowerLayer* cv_PtrOfPowerLayer_get_inner_ptr(const cv::Ptr<cv::dnn::PowerLayer>* instance) {
		return instance->get();
	}

	cv::dnn::PowerLayer* cv_PtrOfPowerLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::PowerLayer>* instance) {
		return instance->get();
	}
}


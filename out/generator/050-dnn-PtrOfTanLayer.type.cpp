extern "C" {
	cv::Ptr<cv::dnn::TanLayer>* cv_PtrOfTanLayer_new(cv::dnn::TanLayer* val) {
		return new cv::Ptr<cv::dnn::TanLayer>(val);
	}
	
	void cv_PtrOfTanLayer_delete(cv::Ptr<cv::dnn::TanLayer>* instance) {
		delete instance;
	}

	const cv::dnn::TanLayer* cv_PtrOfTanLayer_get_inner_ptr(const cv::Ptr<cv::dnn::TanLayer>* instance) {
		return instance->get();
	}

	cv::dnn::TanLayer* cv_PtrOfTanLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::TanLayer>* instance) {
		return instance->get();
	}
}


extern "C" {
	cv::Ptr<cv::dnn::TanHLayer>* cv_PtrOfTanHLayer_new(cv::dnn::TanHLayer* val) {
		return new cv::Ptr<cv::dnn::TanHLayer>(val);
	}
	
	void cv_PtrOfTanHLayer_delete(cv::Ptr<cv::dnn::TanHLayer>* instance) {
		delete instance;
	}

	const cv::dnn::TanHLayer* cv_PtrOfTanHLayer_get_inner_ptr(const cv::Ptr<cv::dnn::TanHLayer>* instance) {
		return instance->get();
	}

	cv::dnn::TanHLayer* cv_PtrOfTanHLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::TanHLayer>* instance) {
		return instance->get();
	}
}


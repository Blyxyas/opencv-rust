extern "C" {
	cv::Ptr<cv::dnn::CosLayer>* cv_PtrOfCosLayer_new(cv::dnn::CosLayer* val) {
		return new cv::Ptr<cv::dnn::CosLayer>(val);
	}
	
	void cv_PtrOfCosLayer_delete(cv::Ptr<cv::dnn::CosLayer>* instance) {
		delete instance;
	}

	const cv::dnn::CosLayer* cv_PtrOfCosLayer_get_inner_ptr(const cv::Ptr<cv::dnn::CosLayer>* instance) {
		return instance->get();
	}

	cv::dnn::CosLayer* cv_PtrOfCosLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::CosLayer>* instance) {
		return instance->get();
	}
}


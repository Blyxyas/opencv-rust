extern "C" {
	cv::Ptr<cv::dnn::AcosLayer>* cv_PtrOfAcosLayer_new(cv::dnn::AcosLayer* val) {
		return new cv::Ptr<cv::dnn::AcosLayer>(val);
	}
	
	void cv_PtrOfAcosLayer_delete(cv::Ptr<cv::dnn::AcosLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AcosLayer* cv_PtrOfAcosLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AcosLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AcosLayer* cv_PtrOfAcosLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AcosLayer>* instance) {
		return instance->get();
	}
}


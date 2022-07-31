extern "C" {
	cv::Ptr<cv::dnn::SoftplusLayer>* cv_PtrOfSoftplusLayer_new(cv::dnn::SoftplusLayer* val) {
		return new cv::Ptr<cv::dnn::SoftplusLayer>(val);
	}
	
	void cv_PtrOfSoftplusLayer_delete(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SoftplusLayer* cv_PtrOfSoftplusLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SoftplusLayer* cv_PtrOfSoftplusLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SoftplusLayer>* instance) {
		return instance->get();
	}
}


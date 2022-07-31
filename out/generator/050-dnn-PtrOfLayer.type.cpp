extern "C" {
	cv::Ptr<cv::dnn::Layer>* cv_PtrOfLayer_new(cv::dnn::Layer* val) {
		return new cv::Ptr<cv::dnn::Layer>(val);
	}
	
	void cv_PtrOfLayer_delete(cv::Ptr<cv::dnn::Layer>* instance) {
		delete instance;
	}

	const cv::dnn::Layer* cv_PtrOfLayer_get_inner_ptr(const cv::Ptr<cv::dnn::Layer>* instance) {
		return instance->get();
	}

	cv::dnn::Layer* cv_PtrOfLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::Layer>* instance) {
		return instance->get();
	}
}


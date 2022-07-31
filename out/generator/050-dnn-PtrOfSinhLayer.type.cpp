extern "C" {
	cv::Ptr<cv::dnn::SinhLayer>* cv_PtrOfSinhLayer_new(cv::dnn::SinhLayer* val) {
		return new cv::Ptr<cv::dnn::SinhLayer>(val);
	}
	
	void cv_PtrOfSinhLayer_delete(cv::Ptr<cv::dnn::SinhLayer>* instance) {
		delete instance;
	}

	const cv::dnn::SinhLayer* cv_PtrOfSinhLayer_get_inner_ptr(const cv::Ptr<cv::dnn::SinhLayer>* instance) {
		return instance->get();
	}

	cv::dnn::SinhLayer* cv_PtrOfSinhLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::SinhLayer>* instance) {
		return instance->get();
	}
}


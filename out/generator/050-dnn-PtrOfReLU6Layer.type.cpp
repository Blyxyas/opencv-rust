extern "C" {
	cv::Ptr<cv::dnn::ReLU6Layer>* cv_PtrOfReLU6Layer_new(cv::dnn::ReLU6Layer* val) {
		return new cv::Ptr<cv::dnn::ReLU6Layer>(val);
	}
	
	void cv_PtrOfReLU6Layer_delete(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		delete instance;
	}

	const cv::dnn::ReLU6Layer* cv_PtrOfReLU6Layer_get_inner_ptr(const cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		return instance->get();
	}

	cv::dnn::ReLU6Layer* cv_PtrOfReLU6Layer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReLU6Layer>* instance) {
		return instance->get();
	}
}


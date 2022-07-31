extern "C" {
	cv::Ptr<cv::dnn::ReshapeLayer>* cv_PtrOfReshapeLayer_new(cv::dnn::ReshapeLayer* val) {
		return new cv::Ptr<cv::dnn::ReshapeLayer>(val);
	}
	
	void cv_PtrOfReshapeLayer_delete(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		delete instance;
	}

	const cv::dnn::ReshapeLayer* cv_PtrOfReshapeLayer_get_inner_ptr(const cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		return instance->get();
	}

	cv::dnn::ReshapeLayer* cv_PtrOfReshapeLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::ReshapeLayer>* instance) {
		return instance->get();
	}
}


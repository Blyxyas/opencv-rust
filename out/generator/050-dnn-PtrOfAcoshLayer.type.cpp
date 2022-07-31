extern "C" {
	cv::Ptr<cv::dnn::AcoshLayer>* cv_PtrOfAcoshLayer_new(cv::dnn::AcoshLayer* val) {
		return new cv::Ptr<cv::dnn::AcoshLayer>(val);
	}
	
	void cv_PtrOfAcoshLayer_delete(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
		delete instance;
	}

	const cv::dnn::AcoshLayer* cv_PtrOfAcoshLayer_get_inner_ptr(const cv::Ptr<cv::dnn::AcoshLayer>* instance) {
		return instance->get();
	}

	cv::dnn::AcoshLayer* cv_PtrOfAcoshLayer_get_inner_ptr_mut(cv::Ptr<cv::dnn::AcoshLayer>* instance) {
		return instance->get();
	}
}


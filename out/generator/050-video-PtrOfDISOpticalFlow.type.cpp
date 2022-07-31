extern "C" {
	void cv_PtrOfDISOpticalFlow_delete(cv::Ptr<cv::DISOpticalFlow>* instance) {
		delete instance;
	}

	const cv::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr(const cv::Ptr<cv::DISOpticalFlow>* instance) {
		return instance->get();
	}

	cv::DISOpticalFlow* cv_PtrOfDISOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::DISOpticalFlow>* instance) {
		return instance->get();
	}
}


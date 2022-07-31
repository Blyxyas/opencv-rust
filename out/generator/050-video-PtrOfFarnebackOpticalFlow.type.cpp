extern "C" {
	void cv_PtrOfFarnebackOpticalFlow_delete(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		delete instance;
	}

	const cv::FarnebackOpticalFlow* cv_PtrOfFarnebackOpticalFlow_get_inner_ptr(const cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}

	cv::FarnebackOpticalFlow* cv_PtrOfFarnebackOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}
}


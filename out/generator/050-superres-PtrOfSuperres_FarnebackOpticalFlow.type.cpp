extern "C" {
	void cv_PtrOfSuperres_FarnebackOpticalFlow_delete(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
		delete instance;
	}

	const cv::superres::FarnebackOpticalFlow* cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr(const cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}

	cv::superres::FarnebackOpticalFlow* cv_PtrOfSuperres_FarnebackOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
		return instance->get();
	}
}


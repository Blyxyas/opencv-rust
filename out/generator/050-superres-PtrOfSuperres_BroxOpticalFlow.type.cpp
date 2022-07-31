extern "C" {
	void cv_PtrOfSuperres_BroxOpticalFlow_delete(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
		delete instance;
	}

	const cv::superres::BroxOpticalFlow* cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr(const cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
		return instance->get();
	}

	cv::superres::BroxOpticalFlow* cv_PtrOfSuperres_BroxOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
		return instance->get();
	}
}


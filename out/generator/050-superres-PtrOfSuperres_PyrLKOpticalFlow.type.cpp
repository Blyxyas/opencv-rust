extern "C" {
	void cv_PtrOfSuperres_PyrLKOpticalFlow_delete(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
		delete instance;
	}

	const cv::superres::PyrLKOpticalFlow* cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr(const cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
		return instance->get();
	}

	cv::superres::PyrLKOpticalFlow* cv_PtrOfSuperres_PyrLKOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
		return instance->get();
	}
}


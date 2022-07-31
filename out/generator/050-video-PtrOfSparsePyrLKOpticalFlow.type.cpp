extern "C" {
	void cv_PtrOfSparsePyrLKOpticalFlow_delete(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		delete instance;
	}

	const cv::SparsePyrLKOpticalFlow* cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr(const cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}

	cv::SparsePyrLKOpticalFlow* cv_PtrOfSparsePyrLKOpticalFlow_get_inner_ptr_mut(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
		return instance->get();
	}
}


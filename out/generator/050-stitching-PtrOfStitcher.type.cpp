extern "C" {
	cv::Ptr<cv::Stitcher>* cv_PtrOfStitcher_new(cv::Stitcher* val) {
		return new cv::Ptr<cv::Stitcher>(val);
	}
	
	void cv_PtrOfStitcher_delete(cv::Ptr<cv::Stitcher>* instance) {
		delete instance;
	}

	const cv::Stitcher* cv_PtrOfStitcher_get_inner_ptr(const cv::Ptr<cv::Stitcher>* instance) {
		return instance->get();
	}

	cv::Stitcher* cv_PtrOfStitcher_get_inner_ptr_mut(cv::Ptr<cv::Stitcher>* instance) {
		return instance->get();
	}
}


extern "C" {
	cv::Ptr<cv::rgbd::DepthCleaner>* cv_PtrOfDepthCleaner_new(cv::rgbd::DepthCleaner* val) {
		return new cv::Ptr<cv::rgbd::DepthCleaner>(val);
	}
	
	void cv_PtrOfDepthCleaner_delete(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		delete instance;
	}

	const cv::rgbd::DepthCleaner* cv_PtrOfDepthCleaner_get_inner_ptr(const cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		return instance->get();
	}

	cv::rgbd::DepthCleaner* cv_PtrOfDepthCleaner_get_inner_ptr_mut(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
		return instance->get();
	}
}


extern "C" {
	cv::Ptr<cv::rgbd::OdometryFrame>* cv_PtrOfOdometryFrame_new(cv::rgbd::OdometryFrame* val) {
		return new cv::Ptr<cv::rgbd::OdometryFrame>(val);
	}
	
	void cv_PtrOfOdometryFrame_delete(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		delete instance;
	}

	const cv::rgbd::OdometryFrame* cv_PtrOfOdometryFrame_get_inner_ptr(const cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		return instance->get();
	}

	cv::rgbd::OdometryFrame* cv_PtrOfOdometryFrame_get_inner_ptr_mut(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
		return instance->get();
	}
}


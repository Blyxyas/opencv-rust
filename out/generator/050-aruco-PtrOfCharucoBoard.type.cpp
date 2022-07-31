extern "C" {
	cv::Ptr<cv::aruco::CharucoBoard>* cv_PtrOfCharucoBoard_new(cv::aruco::CharucoBoard* val) {
		return new cv::Ptr<cv::aruco::CharucoBoard>(val);
	}
	
	void cv_PtrOfCharucoBoard_delete(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		delete instance;
	}

	const cv::aruco::CharucoBoard* cv_PtrOfCharucoBoard_get_inner_ptr(const cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		return instance->get();
	}

	cv::aruco::CharucoBoard* cv_PtrOfCharucoBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::CharucoBoard>* instance) {
		return instance->get();
	}
}


extern "C" {
	cv::Ptr<cv::aruco::Board>* cv_PtrOfBoard_new(cv::aruco::Board* val) {
		return new cv::Ptr<cv::aruco::Board>(val);
	}
	
	void cv_PtrOfBoard_delete(cv::Ptr<cv::aruco::Board>* instance) {
		delete instance;
	}

	const cv::aruco::Board* cv_PtrOfBoard_get_inner_ptr(const cv::Ptr<cv::aruco::Board>* instance) {
		return instance->get();
	}

	cv::aruco::Board* cv_PtrOfBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::Board>* instance) {
		return instance->get();
	}
}


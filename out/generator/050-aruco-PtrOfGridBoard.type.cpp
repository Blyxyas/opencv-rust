extern "C" {
	cv::Ptr<cv::aruco::GridBoard>* cv_PtrOfGridBoard_new(cv::aruco::GridBoard* val) {
		return new cv::Ptr<cv::aruco::GridBoard>(val);
	}
	
	void cv_PtrOfGridBoard_delete(cv::Ptr<cv::aruco::GridBoard>* instance) {
		delete instance;
	}

	const cv::aruco::GridBoard* cv_PtrOfGridBoard_get_inner_ptr(const cv::Ptr<cv::aruco::GridBoard>* instance) {
		return instance->get();
	}

	cv::aruco::GridBoard* cv_PtrOfGridBoard_get_inner_ptr_mut(cv::Ptr<cv::aruco::GridBoard>* instance) {
		return instance->get();
	}
}


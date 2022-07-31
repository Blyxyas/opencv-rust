extern "C" {
	void cv_PtrOfSimpleWB_delete(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		delete instance;
	}

	const cv::xphoto::SimpleWB* cv_PtrOfSimpleWB_get_inner_ptr(const cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		return instance->get();
	}

	cv::xphoto::SimpleWB* cv_PtrOfSimpleWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
		return instance->get();
	}
}


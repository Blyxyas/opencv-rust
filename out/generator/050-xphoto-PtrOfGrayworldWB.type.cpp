extern "C" {
	void cv_PtrOfGrayworldWB_delete(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		delete instance;
	}

	const cv::xphoto::GrayworldWB* cv_PtrOfGrayworldWB_get_inner_ptr(const cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		return instance->get();
	}

	cv::xphoto::GrayworldWB* cv_PtrOfGrayworldWB_get_inner_ptr_mut(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
		return instance->get();
	}
}


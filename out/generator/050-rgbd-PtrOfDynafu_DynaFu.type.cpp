extern "C" {
	void cv_PtrOfDynafu_DynaFu_delete(cv::Ptr<cv::dynafu::DynaFu>* instance) {
		delete instance;
	}

	const cv::dynafu::DynaFu* cv_PtrOfDynafu_DynaFu_get_inner_ptr(const cv::Ptr<cv::dynafu::DynaFu>* instance) {
		return instance->get();
	}

	cv::dynafu::DynaFu* cv_PtrOfDynafu_DynaFu_get_inner_ptr_mut(cv::Ptr<cv::dynafu::DynaFu>* instance) {
		return instance->get();
	}
}


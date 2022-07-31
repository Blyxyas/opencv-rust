extern "C" {
	void cv_PtrOfRetinaFastToneMapping_delete(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
		delete instance;
	}

	const cv::bioinspired::RetinaFastToneMapping* cv_PtrOfRetinaFastToneMapping_get_inner_ptr(const cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
		return instance->get();
	}

	cv::bioinspired::RetinaFastToneMapping* cv_PtrOfRetinaFastToneMapping_get_inner_ptr_mut(cv::Ptr<cv::bioinspired::RetinaFastToneMapping>* instance) {
		return instance->get();
	}
}


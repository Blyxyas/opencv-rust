extern "C" {
	void cv_VectorOfGMat_delete(std::vector<cv::GMat>* instance) {
		delete instance;
	}

	std::vector<cv::GMat>* cv_VectorOfGMat_new() {
		return new std::vector<cv::GMat>();
	}

	size_t cv_VectorOfGMat_len(const std::vector<cv::GMat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfGMat_is_empty(const std::vector<cv::GMat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfGMat_capacity(const std::vector<cv::GMat>* instance) {
		return instance->capacity();
	}

	void cv_<parameter not found>_resize(std::vector<cv::GMat>* instance, size_t new_size) {
		instance->resize(new_size)
	}

	void cv_VectorOfGMat_shrink_to_fit(std::vector<cv::GMat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfGMat_reserve(std::vector<cv::GMat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfGMat_remove(std::vector<cv::GMat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfGMat_swap(std::vector<cv::GMat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfGMat_clear(std::vector<cv::GMat>* instance) {
		instance->clear();
	}

	void cv_VectorOfGMat_push(std::vector<cv::GMat>* instance, cv::GMat* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfGMat_insert(std::vector<cv::GMat>* instance, size_t index, cv::GMat* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfGMat_get(const std::vector<cv::GMat>* instance, size_t index, cv::GMat** ocvrs_return) {
		*ocvrs_return = new cv::GMat((*instance)[index]);
	}

	void cv_VectorOfGMat_set(std::vector<cv::GMat>* instance, size_t index, cv::GMat* val) {
		(*instance)[index] = *val;
	}

}



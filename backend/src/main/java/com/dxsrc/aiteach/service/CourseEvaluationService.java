package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Course;
import java.util.List;

public interface CourseEvaluationService {

    List<Course> list();

    Course getById(Long id);

    boolean save(Course entity);

    boolean update(Course entity);

    boolean removeById(Long id);
}

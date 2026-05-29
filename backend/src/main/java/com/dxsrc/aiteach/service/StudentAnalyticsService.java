package com.dxsrc.aiteach.service;

import com.dxsrc.aiteach.entity.Student;
import java.util.List;

public interface StudentAnalyticsService {

    List<Student> list();

    Student getById(Long id);

    boolean save(Student entity);

    boolean update(Student entity);

    boolean removeById(Long id);
}

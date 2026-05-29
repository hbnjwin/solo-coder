package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Student;
import com.dxsrc.aiteach.mapper.StudentMapper;
import com.dxsrc.aiteach.service.StudentAnalyticsService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class StudentAnalyticsServiceImpl extends ServiceImpl<StudentMapper, Student> implements StudentAnalyticsService {

    @Override
    public List<Student> list() {
        return super.list();
    }

    @Override
    public Student getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Student entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Student entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

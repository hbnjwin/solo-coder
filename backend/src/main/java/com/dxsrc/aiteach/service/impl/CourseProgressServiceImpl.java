package com.dxsrc.aiteach.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.dxsrc.aiteach.entity.Course;
import com.dxsrc.aiteach.mapper.CourseMapper;
import com.dxsrc.aiteach.service.CourseProgressService;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class CourseProgressServiceImpl extends ServiceImpl<CourseMapper, Course> implements CourseProgressService {

    @Override
    public List<Course> list() {
        return super.list();
    }

    @Override
    public Course getById(Long id) {
        return super.getById(id);
    }

    @Override
    public boolean save(Course entity) {
        return super.save(entity);
    }

    @Override
    public boolean update(Course entity) {
        return super.updateById(entity);
    }

    @Override
    public boolean removeById(Long id) {
        return super.removeById(id);
    }
}

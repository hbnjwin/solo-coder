package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Course;
import com.dxsrc.aiteach.service.CourseProgressService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/course-progress")
@RequiredArgsConstructor
public class CourseProgressController {

    private final CourseProgressService courseProgressService;

    @GetMapping("/list")
    public List<Course> list() {
        return courseProgressService.list();
    }

    @GetMapping("/{id}")
    public Course getById(@PathVariable Long id) {
        return courseProgressService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Course entity) {
        return courseProgressService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Course entity) {
        return courseProgressService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return courseProgressService.removeById(id);
    }
}

package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Course;
import com.dxsrc.aiteach.service.CourseEvaluationService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/course-evaluation")
@RequiredArgsConstructor
public class CourseEvaluationController {

    private final CourseEvaluationService courseEvaluationService;

    @GetMapping("/list")
    public List<Course> list() {
        return courseEvaluationService.list();
    }

    @GetMapping("/{id}")
    public Course getById(@PathVariable Long id) {
        return courseEvaluationService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Course entity) {
        return courseEvaluationService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Course entity) {
        return courseEvaluationService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return courseEvaluationService.removeById(id);
    }
}

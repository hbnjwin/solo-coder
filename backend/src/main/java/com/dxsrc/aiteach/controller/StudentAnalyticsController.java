package com.dxsrc.aiteach.controller;

import com.dxsrc.aiteach.entity.Student;
import com.dxsrc.aiteach.service.StudentAnalyticsService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/student-analytics")
@RequiredArgsConstructor
public class StudentAnalyticsController {

    private final StudentAnalyticsService studentAnalyticsService;

    @GetMapping("/list")
    public List<Student> list() {
        return studentAnalyticsService.list();
    }

    @GetMapping("/{id}")
    public Student getById(@PathVariable Long id) {
        return studentAnalyticsService.getById(id);
    }

    @PostMapping
    public boolean save(@RequestBody Student entity) {
        return studentAnalyticsService.save(entity);
    }

    @PutMapping
    public boolean update(@RequestBody Student entity) {
        return studentAnalyticsService.update(entity);
    }

    @DeleteMapping("/{id}")
    public boolean delete(@PathVariable Long id) {
        return studentAnalyticsService.removeById(id);
    }
}
